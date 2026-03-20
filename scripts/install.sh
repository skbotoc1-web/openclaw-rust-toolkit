#!/usr/bin/env bash
set -euo pipefail

REPO="skbotoc1-web/openclaw-rust-toolkit"
BINARY_NAME="octk"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

map_target() {
  case "$OS:$ARCH" in
    Linux:x86_64) echo "x86_64-unknown-linux-gnu" ;;
    Linux:aarch64|Linux:arm64) echo "aarch64-unknown-linux-gnu" ;;
    Darwin:x86_64) echo "x86_64-apple-darwin" ;;
    Darwin:arm64) echo "aarch64-apple-darwin" ;;
    *) return 1 ;;
  esac
}

TARGET="$(map_target || true)"

mkdir -p "$INSTALL_DIR"

install_from_release() {
  if ! command -v curl >/dev/null 2>&1 || ! command -v tar >/dev/null 2>&1; then
    return 1
  fi
  [[ -n "$TARGET" ]] || return 1

  echo "[install] Detect platform target: $TARGET"

  local tag url tmp
  tag="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p' | head -n1)"
  [[ -n "$tag" ]] || return 1

  url="https://github.com/$REPO/releases/download/$tag/${BINARY_NAME}-${TARGET}.tar.gz"
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' RETURN

  echo "[install] Download release: $tag"
  curl -fsSL "$url" -o "$tmp/pkg.tar.gz"
  tar -xzf "$tmp/pkg.tar.gz" -C "$tmp"

  if [[ ! -f "$tmp/$BINARY_NAME" ]]; then
    return 1
  fi

  install -m 0755 "$tmp/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
  echo "[install] Installed $BINARY_NAME to $INSTALL_DIR/$BINARY_NAME"
  return 0
}

install_from_source() {
  echo "[install] Falling back to source install..."

  if ! command -v git >/dev/null 2>&1; then
    echo "[install] git is required for source fallback but was not found."
    return 1
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "[install] Rust not found, installing rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
  fi

  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' RETURN

  git clone --depth 1 "https://github.com/$REPO.git" "$tmp/repo"
  cd "$tmp/repo"
  cargo install --path . --force
}

if ! install_from_release; then
  install_from_source
fi

echo "[install] done."
echo "[install] If '$INSTALL_DIR' is not in PATH, add it first."
echo 'Try: openclaw logs --limit 200 --plain | octk --mode auto --command "openclaw logs --limit 200 --plain" --emit-flag'
