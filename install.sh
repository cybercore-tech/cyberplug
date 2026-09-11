#!/usr/bin/env bash
# Developer rebuild — Omarchy users should prefer:
#   omarchy plugin add https://github.com/darkstardevx/cyberplug.git --enable
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

echo "Building cyberplug (release)..."
cargo build --release

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  BIN_SRC="${CARGO_TARGET_DIR}/release/cyberplug"
elif [[ -f "$HOME/.cargo/config.toml" ]] && grep -q "target-dir" "$HOME/.cargo/config.toml"; then
  TARGET_DIR=$(grep "target-dir" "$HOME/.cargo/config.toml" | sed -E 's/.*=\s*"(.*)"/\1/')
  BIN_SRC="${TARGET_DIR}/release/cyberplug"
else
  BIN_SRC="target/release/cyberplug"
fi

if [[ ! -f "$BIN_SRC" ]]; then
  echo "Could not find built binary at: $BIN_SRC"
  exit 1
fi

case "$(uname -m)" in
  x86_64 | amd64) ARCH_DIR="linux-x86_64" ;;
  aarch64 | arm64) ARCH_DIR="linux-aarch64" ;;
  *) ARCH_DIR="linux-$(uname -m)" ;;
esac

PLUGIN_BIN_DIR="$ROOT/bin/$ARCH_DIR"
mkdir -p "$PLUGIN_BIN_DIR"
strip -o "$PLUGIN_BIN_DIR/cyberplug" "$BIN_SRC" 2>/dev/null \
  || cp "$BIN_SRC" "$PLUGIN_BIN_DIR/cyberplug"
chmod +x "$PLUGIN_BIN_DIR/cyberplug"
echo "Bundled plugin binary → $PLUGIN_BIN_DIR/cyberplug"

if [[ "${1:-}" == "--local-bin" ]]; then
  mkdir -p "$HOME/.local/bin"
  cp "$PLUGIN_BIN_DIR/cyberplug" "$HOME/.local/bin/cyberplug"
  chmod +x "$HOME/.local/bin/cyberplug"
  echo "Also installed → $HOME/.local/bin/cyberplug"
fi

echo "Done. Omarchy install:"
echo "  omarchy plugin add https://github.com/darkstardevx/cyberplug.git --enable"
