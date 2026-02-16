import type { PluginAPI } from "openclaw";

const MODEL_NAME = "sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17";
const MODEL_URL =
  "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17.tar.bz2";
const MODEL_DIR_RELATIVE = `models/${MODEL_NAME}`;
const AUDIO_EXTS = [".wav", ".mp3", ".m4a", ".ogg", ".flac", ".aac", ".opus"];

async function findCliPath(api: PluginAPI): Promise<string | null> {
  const home = process.env.HOME || process.env.USERPROFILE || "~";
  const paths = [
    `${api.pluginDir}/rust/target/release/sensevoice-cli`,
    `${home}/.openclaw/bin/sensevoice-cli`,
  ];
  for (const p of paths) {
    try {
      await api.runtime.system.runCommandWithTimeout("test", ["-x", p], {
        timeoutMs: 5000,
      });
      return p;
    } catch {}
  }
  return null;
}

export default function activate(api: PluginAPI) {
  // Auto-transcribe voice messages
  api.onMessage(async (message) => {
    const audioAttachments = (message.attachments || []).filter(
      (a: any) =>
        a.mimeType?.startsWith("audio/") ||
        AUDIO_EXTS.some((ext) => a.filePath?.toLowerCase().endsWith(ext))
    );

    if (audioAttachments.length === 0) return;

    const cliPath = await findCliPath(api);
    if (!cliPath) {
      api.log("sensevoice-cli not found, skipping transcription. Run /sensevoice setup first.");
      return;
    }

    const transcriptions: string[] = [];
    for (const attachment of audioAttachments) {
      try {
        const result = await api.runtime.system.runCommandWithTimeout(
          cliPath,
          [attachment.filePath],
          { timeoutMs: 60000 }
        );
        if (result.stdout.trim()) {
          transcriptions.push(result.stdout.trim());
        }
      } catch (e: any) {
        api.log(`Transcription failed for ${attachment.filePath}: ${e.message}`);
      }
    }

    if (transcriptions.length > 0) {
      message.content = transcriptions.join("\n");
    }
  });

  // Slash commands
  api.registerCommand("sensevoice", {
    description: "Manage SenseVoice offline speech-to-text",
    usage: "/sensevoice <setup|status|test>",
    async execute(args: string) {
      const sub = args.trim().split(/\s+/);
      const cmd = sub[0] || "status";

      switch (cmd) {
        case "setup":
          return await setup(api);
        case "status":
          return await status(api);
        case "test":
          return await test(api, sub.slice(1).join(" "));
        default:
          return `Unknown subcommand: ${cmd}\nUsage: /sensevoice <setup|status|test>`;
      }
    },
  });
}

async function setup(api: PluginAPI) {
  const home = process.env.HOME || process.env.USERPROFILE || "~";
  const modelDir = `${home}/.openclaw/${MODEL_DIR_RELATIVE}`;
  const modelFile = `${modelDir}/model.int8.onnx`;

  // Check if model already exists
  try {
    await api.runtime.system.runCommandWithTimeout("test", ["-f", modelFile], {
      timeoutMs: 5000,
    });
    api.log("Model already downloaded, skipping download.");
  } catch {
    // Download model
    api.log("Downloading SenseVoice model (~228MB needed from ~1.1GB archive)...");
    api.log("This may take a few minutes depending on your connection.");

    await api.runtime.system.runCommandWithTimeout("mkdir", ["-p", modelDir], {
      timeoutMs: 5000,
    });

    // Download and extract only the needed files (model.int8.onnx + tokens.txt)
    const extractDir = `${home}/.openclaw/models`;
    try {
      await api.runtime.system.runCommandWithTimeout(
        "bash",
        [
          "-c",
          `curl -fSL "${MODEL_URL}" | tar xj -C "${extractDir}" --include='*model.int8.onnx' --include='*tokens.txt'`,
        ],
        { timeoutMs: 600000 } // 10 min timeout for download
      );
      api.log("Model downloaded successfully.");
    } catch (e: any) {
      return `Failed to download model: ${e.message}\nYou can manually download from: ${MODEL_URL}`;
    }
  }

  // Patch openclaw.json audio config
  try {
    const config = await api.runtime.config.loadConfig();

    const cliPath = await findCliPath(api);
    if (!cliPath) {
      return `Model downloaded but sensevoice-cli binary not found.\nPlease build from source or download from GitHub Releases.`;
    }

    // Update tools.media.audio config
    const audioConfig = {
      type: "cli" as const,
      command: cliPath,
      args: ["$1"],
    };

    if (!config.tools) config.tools = {};
    if (!config.tools.media) config.tools.media = {};
    if (!config.tools.media.audio) config.tools.media.audio = { enabled: true, models: [] };

    config.tools.media.audio.enabled = true;

    // Replace existing sensevoice-cli entry or add new one
    const models = config.tools.media.audio.models || [];
    const idx = models.findIndex(
      (m: any) => m.type === "cli" && m.command?.includes("sensevoice-cli")
    );
    if (idx >= 0) {
      models[idx] = audioConfig;
    } else {
      models.push(audioConfig);
    }
    config.tools.media.audio.models = models;

    await api.runtime.config.writeConfigFile(config);
    api.log("openclaw.json updated with SenseVoice audio config.");
  } catch (e: any) {
    api.log(`Warning: could not update openclaw.json: ${e.message}`);
  }

  return "SenseVoice setup complete! Voice messages will now be auto-transcribed.\nUse `/sensevoice status` to verify.";
}

async function status(api: PluginAPI) {
  const home = process.env.HOME || process.env.USERPROFILE || "~";
  const lines: string[] = [];

  // Check binary
  const cliPath = await findCliPath(api);
  if (cliPath) {
    lines.push(`Binary: ${cliPath} (OK)`);
  } else {
    lines.push("Binary: NOT FOUND");
  }

  // Check model
  const modelDir = `${home}/.openclaw/models/${MODEL_NAME}`;
  const modelFile = `${modelDir}/model.int8.onnx`;
  const tokensFile = `${modelDir}/tokens.txt`;
  try {
    await api.runtime.system.runCommandWithTimeout("test", ["-f", modelFile], {
      timeoutMs: 5000,
    });
    lines.push(`Model: ${modelFile} (OK)`);
  } catch {
    lines.push(`Model: NOT FOUND at ${modelFile}`);
  }
  try {
    await api.runtime.system.runCommandWithTimeout(
      "test",
      ["-f", tokensFile],
      { timeoutMs: 5000 }
    );
    lines.push(`Tokens: ${tokensFile} (OK)`);
  } catch {
    lines.push(`Tokens: NOT FOUND at ${tokensFile}`);
  }

  // Check openclaw.json config
  try {
    const config = await api.runtime.config.loadConfig();
    const hasAudio =
      config?.tools?.media?.audio?.models?.some(
        (m: any) => m.type === "cli" && m.command?.includes("sensevoice-cli")
      ) ?? false;
    lines.push(`Config: ${hasAudio ? "Configured in openclaw.json" : "NOT configured"}`);
  } catch {
    lines.push("Config: Could not read openclaw.json");
  }

  lines.push(`\nAuto-transcription: ${cliPath ? "ACTIVE" : "INACTIVE (binary not found)"}`);

  return lines.join("\n");
}

async function test(api: PluginAPI, filePath: string) {
  if (!filePath) {
    return "Usage: /sensevoice test <audio_file_path>";
  }

  const cliPath = await findCliPath(api);
  if (!cliPath) {
    return "sensevoice-cli binary not found. Run `/sensevoice setup` first.";
  }

  try {
    const result = await api.runtime.system.runCommandWithTimeout(
      cliPath,
      [filePath],
      { timeoutMs: 60000 }
    );
    return `Transcription:\n${result.stdout}`;
  } catch (e: any) {
    return `Transcription failed: ${e.message}`;
  }
}
