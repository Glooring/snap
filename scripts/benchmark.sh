#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'HELP'
Usage: bash scripts/benchmark.sh [--snap-bin PATH] [--workdir DIR] [--keep] [--help]

Runs a local Snap benchmark smoke test in a disposable Git repository.

Options:
  --snap-bin PATH  Use an existing source-built Snap binary.
  --workdir DIR    Benchmark workspace root. Defaults to target/snap-benchmarks.
  --keep           Keep the disposable benchmark repository after the run.
  --help           Print this help text.
HELP
}

SNAP_BIN=""
WORK_ROOT=""
KEEP=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --snap-bin)
      SNAP_BIN="${2:-}"
      shift 2
      ;;
    --workdir)
      WORK_ROOT="${2:-}"
      shift 2
      ;;
    --keep)
      KEEP=1
      shift
      ;;
    --help|-h)
      show_help
      exit 0
      ;;
    *)
      echo "[snap-bench] Unknown option: $1" >&2
      show_help >&2
      exit 2
      ;;
  esac
done

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

if [[ -z "$SNAP_BIN" ]]; then
  cargo build --release
  SNAP_BIN="$REPO_ROOT/target/release/snap"
fi

if [[ ! -x "$SNAP_BIN" ]]; then
  echo "[snap-bench] Snap binary is not executable: $SNAP_BIN" >&2
  exit 1
fi

if [[ -z "$WORK_ROOT" ]]; then
  WORK_ROOT="$REPO_ROOT/target/snap-benchmarks"
fi

mkdir -p "$WORK_ROOT"
BENCH_DIR="$WORK_ROOT/bench-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BENCH_DIR"

cleanup() {
  if [[ "$KEEP" -eq 0 ]]; then
    rm -rf "$BENCH_DIR"
  else
    echo "[snap-bench] Kept benchmark repository: $BENCH_DIR"
  fi
}
trap cleanup EXIT

now_ns() {
  date +%s%N
}

run_timed() {
  local label="$1"
  shift
  local start
  local end
  local duration_ms
  start="$(now_ns)"
  "$@" >/dev/null
  end="$(now_ns)"
  duration_ms=$(( (end - start) / 1000000 ))
  printf '%-28s %8d ms\n' "$label" "$duration_ms"
}

echo "[snap-bench] Snap binary: $SNAP_BIN"
echo "[snap-bench] Repository: $BENCH_DIR"

cd "$BENCH_DIR"
git init >/dev/null
git config user.email "snap-benchmark@example.invalid"
git config user.name "Snap Benchmark"

mkdir -p "src/module with spaces" "data/unicode-λ" "metadata/nested/empty/leaf"
for i in $(seq 1 250); do
  printf 'line %03d\n' "$i" > "src/file-$i.txt"
done
printf 'space path\n' > "src/module with spaces/file one.txt"
printf 'unicode path\n' > "data/unicode-λ/naïve-Δ.txt"
printf 'hidden\n' > ".hidden-benchmark"
chmod 0444 "src/file-250.txt"

git add .
git commit -m "initial benchmark content" >/dev/null

printf '\n%-28s %s\n' "command" "duration"
printf '%-28s %s\n' "-------" "--------"
run_timed "snap init" "$SNAP_BIN" init
run_timed "snap new baseline" "$SNAP_BIN" new bench-baseline --include-metadata-only "baseline benchmark snapshot"

for i in $(seq 1 50); do
  printf 'changed %03d\n' "$i" >> "src/file-$i.txt"
done
mkdir -p "metadata/new empty/child"
printf 'new file\n' > "src/module with spaces/new file.txt"
run_timed "snap new changed" "$SNAP_BIN" new bench-changed --include-metadata-only "changed benchmark snapshot"
run_timed "snap list" "$SNAP_BIN" list
run_timed "snap diff" "$SNAP_BIN" diff bench-baseline bench-changed
run_timed "snap doctor" "$SNAP_BIN" doctor
run_timed "restore dry-run" "$SNAP_BIN" restore bench-baseline --dry-run

printf '\n[snap-bench] Complete. Results are local observations only.\n'
