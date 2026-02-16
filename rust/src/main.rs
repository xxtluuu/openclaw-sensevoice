mod audio;
mod transcript;

use anyhow::{Context, Result};
use sherpa_rs::sense_voice::{SenseVoiceConfig, SenseVoiceRecognizer};
use std::path::PathBuf;

const MODEL_SUBDIR: &str = "sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17";

/// Resolve model directory using three-level lookup:
/// 1. SENSEVOICE_MODEL_DIR environment variable
/// 2. <binary_dir>/../model/<MODEL_SUBDIR>/  (plugin layout)
/// 3. ~/.openclaw/models/<MODEL_SUBDIR>/      (standard location)
fn resolve_model_dir() -> Result<PathBuf> {
    // 1. Environment variable
    if let Ok(dir) = std::env::var("SENSEVOICE_MODEL_DIR") {
        let p = PathBuf::from(&dir);
        if p.join("model.int8.onnx").exists() {
            return Ok(p);
        }
        eprintln!("warn: SENSEVOICE_MODEL_DIR={dir} does not contain model.int8.onnx, trying other locations");
    }

    // 2. Binary-relative: <binary_dir>/../model/<MODEL_SUBDIR>/
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            let plugin_model = bin_dir.join("..").join("model").join(MODEL_SUBDIR);
            if plugin_model.join("model.int8.onnx").exists() {
                return Ok(plugin_model);
            }
        }
    }

    // 3. ~/.openclaw/models/<MODEL_SUBDIR>/
    if let Some(home) = dirs::home_dir() {
        let standard = home.join(".openclaw").join("models").join(MODEL_SUBDIR);
        if standard.join("model.int8.onnx").exists() {
            return Ok(standard);
        }
    }

    anyhow::bail!(
        "SenseVoice model not found. Searched:\n\
         1. $SENSEVOICE_MODEL_DIR\n\
         2. <binary_dir>/../model/{MODEL_SUBDIR}/\n\
         3. ~/.openclaw/models/{MODEL_SUBDIR}/\n\
         \n\
         Run `/sensevoice setup` or set SENSEVOICE_MODEL_DIR to the model directory."
    )
}

fn main() -> Result<()> {
    let audio_path = std::env::args()
        .nth(1)
        .context("Usage: sensevoice-cli <audio_file>")?;

    let model_dir = resolve_model_dir()?;
    let model_path = model_dir.join("model.int8.onnx");
    let tokens_path = model_dir.join("tokens.txt");

    // Verify files exist
    if !model_path.exists() {
        anyhow::bail!("Model file not found: {}", model_path.display());
    }
    if !tokens_path.exists() {
        anyhow::bail!("Tokens file not found: {}", tokens_path.display());
    }

    // Load and decode audio to 16kHz mono f32
    let samples = audio::load_audio(&audio_path)?;

    // Initialize recognizer
    let config = SenseVoiceConfig {
        model: model_path.to_string_lossy().into_owned(),
        tokens: tokens_path.to_string_lossy().into_owned(),
        language: "auto".into(),
        use_itn: true,
        num_threads: Some(4),
        debug: false,
        provider: Some("cpu".into()),
    };

    let mut recognizer = SenseVoiceRecognizer::new(config)
        .map_err(|e| anyhow::anyhow!("Failed to create SenseVoice recognizer: {e}"))?;

    // Transcribe
    let result = recognizer.transcribe(16000, &samples);

    // Clean up markers and output
    let text = transcript::clean_transcript(&result.text);
    print!("{text}");

    Ok(())
}
