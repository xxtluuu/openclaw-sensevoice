#!/usr/bin/env bash
set -euo pipefail

REPO="xxtluuu/openclaw-sensevoice"
VERSION="${SENSEVOICE_VERSION:-$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"v\(.*\)".*/\1/')}"

if [[ -z "$VERSION" ]]; then
  error "Failed to detect latest version. Set SENSEVOICE_VERSION manually."
  exit 1
fi
MODEL_NAME="sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17"
MODEL_URL="https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/${MODEL_NAME}.tar.bz2"
INSTALL_DIR="${HOME}/.openclaw"
BIN_DIR="${INSTALL_DIR}/bin"
MODEL_DIR="${INSTALL_DIR}/models/${MODEL_NAME}"

info()  { printf '\033[1;34m[info]\033[0m  %s\n' "$*"; }
ok()    { printf '\033[1;32m[ok]\033[0m    %s\n' "$*"; }
warn()  { printf '\033[1;33m[warn]\033[0m  %s\n' "$*"; }
error() { printf '\033[1;31m[error]\033[0m %s\n' "$*" >&2; }

# --- Detect platform ---
detect_platform() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"

  case "$os" in
    darwin) os="darwin" ;;
    linux)  os="linux"  ;;
    *)      error "Unsupported OS: $os"; exit 1 ;;
  esac

  case "$arch" in
    arm64|aarch64) arch="arm64"  ;;
    x86_64|amd64)  arch="x86_64" ;;
    *)              error "Unsupported architecture: $arch"; exit 1 ;;
  esac

  echo "${os}-${arch}"
}

# --- Download and install binary ---
install_binary() {
  local platform="$1"
  local archive="sensevoice-cli-${VERSION}-${platform}.tar.gz"
  local url="https://github.com/${REPO}/releases/download/v${VERSION}/${archive}"

  info "Downloading sensevoice-cli ${VERSION} for ${platform}..."
  mkdir -p "${BIN_DIR}"

  local tmpdir
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT

  curl -fSL "$url" -o "${tmpdir}/${archive}"

  # SHA256 checksum verification
  local checksum_url="https://github.com/${REPO}/releases/download/v${VERSION}/checksums-sha256.txt"
  if curl -fSL "$checksum_url" -o "${tmpdir}/checksums-sha256.txt" 2>/dev/null; then
    info "Verifying checksum..."
    local expected
    expected="$(grep "${archive}" "${tmpdir}/checksums-sha256.txt" | awk '{print $1}')"
    local actual
    if command -v sha256sum &>/dev/null; then
      actual="$(sha256sum "${tmpdir}/${archive}" | awk '{print $1}')"
    else
      actual="$(shasum -a 256 "${tmpdir}/${archive}" | awk '{print $1}')"
    fi
    if [[ "$expected" != "$actual" ]]; then
      error "Checksum mismatch! Expected: ${expected}, Got: ${actual}"
      exit 1
    fi
    ok "Checksum verified"
  else
    warn "Checksum file not available, skipping verification"
  fi

  tar xzf "${tmpdir}/${archive}" -C "${tmpdir}"

  # Copy binary and libraries
  local extracted="${tmpdir}/sensevoice-cli-${VERSION}-${platform}/bin"
  cp "${extracted}/sensevoice-cli" "${BIN_DIR}/"
  chmod +x "${BIN_DIR}/sensevoice-cli"

  # Copy dynamic libraries
  cp "${extracted}"/lib* "${BIN_DIR}/" 2>/dev/null || true

  # macOS: remove quarantine attributes
  if [[ "$platform" == darwin-* ]]; then
    xattr -cr "${BIN_DIR}/sensevoice-cli" 2>/dev/null || true
    xattr -cr "${BIN_DIR}"/lib* 2>/dev/null || true
  fi

  ok "Binary installed to ${BIN_DIR}/sensevoice-cli"
}

# --- Download model ---
install_model() {
  if [[ -f "${MODEL_DIR}/model.int8.onnx" && -f "${MODEL_DIR}/tokens.txt" ]]; then
    ok "Model already exists at ${MODEL_DIR}"
    return
  fi

  info "Downloading SenseVoice model (~228MB from ~1.1GB archive)..."
  info "This may take a few minutes."
  mkdir -p "${INSTALL_DIR}/models"

  curl -fSL "$MODEL_URL" | tar xj -C "${INSTALL_DIR}/models" \
    --include="*model.int8.onnx" \
    --include="*tokens.txt"

  if [[ -f "${MODEL_DIR}/model.int8.onnx" ]]; then
    ok "Model installed to ${MODEL_DIR}"
  else
    error "Model extraction failed"
    exit 1
  fi
}

# --- Print config guidance ---
print_guidance() {
  echo ""
  info "Installation complete!"
  echo ""
  echo "  Binary:  ${BIN_DIR}/sensevoice-cli"
  echo "  Model:   ${MODEL_DIR}/"
  echo ""
  echo "  To configure OpenClaw, add to ~/.openclaw/openclaw.json:"
  echo ""
  echo '  "tools": {'
  echo '    "media": {'
  echo '      "audio": {'
  echo '        "enabled": true,'
  echo '        "models": [{'
  echo '          "type": "cli",'
  echo "          \"command\": \"${BIN_DIR}/sensevoice-cli\","
  echo '          "args": ["$1"]'
  echo '        }]'
  echo '      }'
  echo '    }'
  echo '  }'
  echo ""
  echo "  Or install as OpenClaw plugin and run: /sensevoice setup"
  echo ""
}

# --- Main ---
main() {
  info "Installing openclaw-sensevoice v${VERSION}"
  echo ""

  local platform
  platform="$(detect_platform)"
  info "Detected platform: ${platform}"

  install_binary "$platform"
  install_model
  print_guidance
}

main "$@"
