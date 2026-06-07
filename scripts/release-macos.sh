#!/usr/bin/env bash
set -euo pipefail

# Creates macOS release assets for GitHub Releases.
# The script builds the native macOS architecture for the current runner.
APP_NAME="${APP_NAME:-snap}"
RELEASE_ROOT="${RELEASE_ROOT:-release-github}"
MACOS_TARGET="${MACOS_TARGET:-}"

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

require_cmd() {
  local name="$1"
  local hint="$2"

  if ! command -v "$name" >/dev/null 2>&1; then
    echo "[snap-release] Required command '$name' was not found. $hint" >&2
    exit 1
  fi
}

run_step() {
  local label="$1"
  shift

  echo
  echo "[snap-release] $label"
  "$@"
}

require_cmd cargo "Install Rust from https://rustup.rs/."
require_cmd tar "Install tar."
require_cmd uname "Install standard Unix core utilities."

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "[snap-release] macOS release assets must be built on macOS." >&2
  exit 1
fi

ARCH="$(uname -m)"
if [[ -z "$MACOS_TARGET" ]]; then
  case "$ARCH" in
    arm64) MACOS_TARGET="macos-aarch64" ;;
    x86_64) MACOS_TARGET="macos-x86_64" ;;
    *)
      echo "[snap-release] Unsupported macOS architecture: $ARCH" >&2
      exit 1
      ;;
  esac
fi

case "$MACOS_TARGET:$ARCH" in
  macos-aarch64:arm64) ;;
  macos-x86_64:x86_64) ;;
  macos-aarch64:*|macos-x86_64:*)
    echo "[snap-release] MACOS_TARGET '$MACOS_TARGET' does not match host architecture '$ARCH'." >&2
    echo "[snap-release] Build macos-aarch64 on Apple Silicon and macos-x86_64 on Intel macOS." >&2
    exit 1
    ;;
  *)
    echo "[snap-release] Unsupported MACOS_TARGET '$MACOS_TARGET'. Expected macos-aarch64 or macos-x86_64." >&2
    exit 1
    ;;
esac

VERSION="$(cargo pkgid | sed 's/.*#//')"
if [[ -z "$VERSION" ]]; then
  echo "[snap-release] Failed to read package version with 'cargo pkgid'." >&2
  exit 1
fi

RELEASE_VERSION="v$VERSION"
if [[ "$RELEASE_ROOT" = /* ]]; then
  RELEASE_ROOT_PATH="$RELEASE_ROOT"
else
  RELEASE_ROOT_PATH="$REPO_ROOT/$RELEASE_ROOT"
fi
RELEASE_DIR="$RELEASE_ROOT_PATH/$RELEASE_VERSION"
mkdir -p "$RELEASE_DIR"

SOURCE_BIN="$REPO_ROOT/target/release/$APP_NAME"
RELEASE_BIN="$RELEASE_DIR/$APP_NAME-$RELEASE_VERSION-$MACOS_TARGET"
RELEASE_ARCHIVE="$RELEASE_DIR/$APP_NAME-$RELEASE_VERSION-$MACOS_TARGET.tar.gz"
PACKAGE_DIR="$REPO_ROOT/target/macos-dist/$MACOS_TARGET/package"

run_step "Running macOS test suite" cargo test
run_step "Building macOS release binary" cargo build --release

if [[ ! -f "$SOURCE_BIN" ]]; then
  echo "[snap-release] Expected release binary was not found: $SOURCE_BIN" >&2
  exit 1
fi

cp "$SOURCE_BIN" "$RELEASE_BIN"
chmod +x "$RELEASE_BIN"

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR"
cp "$SOURCE_BIN" "$PACKAGE_DIR/$APP_NAME"
chmod +x "$PACKAGE_DIR/$APP_NAME"
tar -czf "$RELEASE_ARCHIVE" -C "$PACKAGE_DIR" "$APP_NAME"

if [[ ! -f "$RELEASE_BIN" || ! -f "$RELEASE_ARCHIVE" ]]; then
  echo "[snap-release] One or more macOS release artifacts were not created." >&2
  exit 1
fi

run_step "Verifying macOS binary version" "$RELEASE_BIN" --version

echo
echo "[snap-release] macOS release assets are ready:"
echo "  $RELEASE_BIN"
echo "  $RELEASE_ARCHIVE"
