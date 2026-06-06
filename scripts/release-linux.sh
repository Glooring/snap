#!/usr/bin/env bash
set -euo pipefail

# Creates Linux release assets for GitHub Releases.
# Edit these defaults if the release layout or target name changes.
APP_NAME="${APP_NAME:-snap}"
RELEASE_ROOT="${RELEASE_ROOT:-release-github}"
LINUX_TARGET="${LINUX_TARGET:-linux-x86_64}"

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
require_cmd tar "Install tar with your Linux distribution package manager."

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
RELEASE_BIN="$RELEASE_DIR/$APP_NAME-$RELEASE_VERSION-$LINUX_TARGET"
RELEASE_ARCHIVE="$RELEASE_DIR/$APP_NAME-$RELEASE_VERSION-$LINUX_TARGET.tar.gz"
PACKAGE_DIR="$REPO_ROOT/target/linux-dist/package"

run_step "Running Linux test suite" cargo test
run_step "Building Linux release binary" cargo build --release

if [[ ! -f "$SOURCE_BIN" ]]; then
  echo "[snap-release] Expected release binary was not found: $SOURCE_BIN" >&2
  exit 1
fi

cp "$SOURCE_BIN" "$RELEASE_BIN"
chmod +x "$RELEASE_BIN"

mkdir -p "$PACKAGE_DIR"
cp "$SOURCE_BIN" "$PACKAGE_DIR/$APP_NAME"
chmod +x "$PACKAGE_DIR/$APP_NAME"
tar -czf "$RELEASE_ARCHIVE" -C "$PACKAGE_DIR" "$APP_NAME"

if [[ ! -f "$RELEASE_BIN" || ! -f "$RELEASE_ARCHIVE" ]]; then
  echo "[snap-release] One or more Linux release artifacts were not created." >&2
  exit 1
fi

run_step "Verifying Linux binary version" "$RELEASE_BIN" --version

echo
echo "[snap-release] Linux release assets are ready:"
echo "  $RELEASE_BIN"
echo "  $RELEASE_ARCHIVE"
