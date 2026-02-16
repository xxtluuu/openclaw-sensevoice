use anyhow::{Context, Result};
use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

const TARGET_SAMPLE_RATE: u32 = 16_000;

/// Load an audio file (any format symphonia supports), decode to mono f32 at 16kHz.
pub fn load_audio(path: &str) -> Result<Vec<f32>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("Cannot open audio file: {path}"))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Provide file extension hint for format detection
    let mut hint = Hint::new();
    if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .context("Unsupported or invalid audio format")?;

    let mut format = probed.format;

    // Find the first audio track
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .context("No audio track found")?;

    let codec_params = track.codec_params.clone();
    let track_id = track.id;
    let channels = codec_params.channels.map(|c| c.count()).unwrap_or(1);
    let sample_rate = codec_params
        .sample_rate
        .context("Audio track has no sample rate")?;

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .context("Unsupported codec")?;

    // Decode all packets into interleaved f32
    let mut all_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(e.into()),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet)?;
        let spec = *decoded.spec();
        let duration = decoded.capacity();

        let mut sample_buf = SampleBuffer::<f32>::new(duration as u64, spec);
        sample_buf.copy_interleaved_ref(decoded);
        all_samples.extend_from_slice(sample_buf.samples());
    }

    if all_samples.is_empty() {
        anyhow::bail!("No audio samples decoded from {path}");
    }

    // Mix down to mono if multi-channel
    let mono = if channels > 1 {
        mix_to_mono(&all_samples, channels)
    } else {
        all_samples
    };

    // Resample to 16kHz if needed
    if sample_rate == TARGET_SAMPLE_RATE {
        Ok(mono)
    } else {
        resample(&mono, sample_rate, TARGET_SAMPLE_RATE)
    }
}

/// Average interleaved multi-channel samples into mono.
fn mix_to_mono(interleaved: &[f32], channels: usize) -> Vec<f32> {
    let n_frames = interleaved.len() / channels;
    let inv = 1.0 / channels as f32;
    let mut mono = Vec::with_capacity(n_frames);
    for i in 0..n_frames {
        let mut sum = 0.0f32;
        for ch in 0..channels {
            sum += interleaved[i * channels + ch];
        }
        mono.push(sum * inv);
    }
    mono
}

/// Resample mono f32 audio using rubato (sinc interpolation).
fn resample(input: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };

    let ratio = to_rate as f64 / from_rate as f64;
    let mut resampler = SincFixedIn::<f32>::new(
        ratio,
        2.0,       // max relative ratio (fixed, so doesn't matter much)
        params,
        input.len(),
        1, // mono
    )?;

    let output = resampler.process(&[input], None)?;
    Ok(output.into_iter().next().unwrap_or_default())
}
