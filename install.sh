#!/usr/bin/env bash
# Developer/CLI helper — builds a release binary and optionally installs it
# to ~/.local/bin for running `cyberplug` outside the Omarchy bar widget.
#
# The bar widget itself does NOT need this: cyberplug-toggle builds and
# caches its own binary under bin/<arch>/ on first launch. Omarchy users
# should just do:
#   omarchy plugin add https://github.com/darkstardevx/cyberplug.git --enable
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
cd "$ROOT"

echo "Building cyberplug (release)..."
cargo build --release --locked

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

echo "Built → $BIN_SRC"

if [[ "${1:-}" == "--local-bin" ]]; then
  mkdir -p "$HOME/.local/bin"
  tmp_bin="$HOME/.local/bin/cyberplug.tmp.$$"
  cp "$BIN_SRC" "$tmp_bin"
  chmod +x "$tmp_bin"
  mv -f "$tmp_bin" "$HOME/.local/bin/cyberplug"
  echo "Installed → $HOME/.local/bin/cyberplug"
fi

echo "Done. Omarchy bar widget install:"
echo "  omarchy plugin add https://github.com/darkstardevx/cyberplug.git --enable"
