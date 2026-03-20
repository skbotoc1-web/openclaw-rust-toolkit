#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "[install] Rust not found, installing rustup..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

echo "[install] Installing octk from GitHub source..."
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

git clone --depth 1 https://github.com/skbotoc1-web/openclaw-rust-toolkit.git "$TMP_DIR/openclaw-rust-toolkit"
cd "$TMP_DIR/openclaw-rust-toolkit"
cargo install --path . --force

echo "[install] done. Try:"
echo 'openclaw logs --limit 200 --plain | octk --mode auto --command "openclaw logs --limit 200 --plain" --emit-flag'
