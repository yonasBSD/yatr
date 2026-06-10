#!/usr/bin/env bash
# Reproducible macro-benchmark: yatr vs make / just / task.
#
# Times two stable scenarios on an identical workload:
#   1. startup     — start up + parse config + decide there's nothing to do
#   2. warm-noop   — re-run a cached/up-to-date "build" (the cache's headline)
#
# Tools that aren't installed are skipped. Numbers are min / mean of repeated
# runs (wall clock, ms). Run from anywhere:  benches/bench.sh
set -euo pipefail

# --- locate a release yatr -------------------------------------------------
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
YATR="${YATR:-$REPO/target/release/yatr}"
if [[ ! -x "$YATR" ]]; then
  echo "Building release yatr…" >&2
  (cd "$REPO" && cargo build --release --quiet)
fi
have() { command -v "$1" >/dev/null 2>&1; }

# --- timer: prints "min mean" ms over N runs of a shell command ------------
timeit() {
  CMD="$1" python3 - <<'PY'
import os, subprocess, time
cmd = os.environ["CMD"]
run = lambda: subprocess.run(cmd, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
run()  # warm up
ts = []
for _ in range(12):
    t = time.perf_counter(); run(); ts.append((time.perf_counter() - t) * 1000)
ts.sort()
print(f"{ts[0]:.1f} {sum(ts)/len(ts):.1f}")
PY
}

# --- generate an identical benchmark project for each tool -----------------
DIR="$(mktemp -d)"
trap 'rm -rf "$DIR"' EXIT
cd "$DIR"
echo "source v1" > input.txt
WORK="sh -c 'sleep 0.1 && cp input.txt out.txt'"   # ~100ms of "compilation"

cat > yatr.toml <<EOF
[settings]
cache_dir = ".yatrcache"
[tasks.build]
sources = ["input.txt"]
outputs = ["out.txt"]
run = ["$WORK"]
EOF

printf 'build: out.txt\nout.txt: input.txt\n\t%s\n' "$WORK" > Makefile

printf 'build:\n\t%s\n' "$WORK" > justfile

cat > Taskfile.yml <<EOF
version: '3'
tasks:
  build:
    sources: [input.txt]
    generates: [out.txt]
    cmds: ["$WORK"]
EOF

# --- run -------------------------------------------------------------------
row() { printf "| %-22s | %18s | %18s |\n" "$1" "$2" "$3"; }
sep() { printf "|%s|%s|%s|\n" "------------------------" "--------------------" "--------------------"; }

echo
echo "### Results (min / mean ms — lower is better)"
echo
row "Tool" "startup (noop)" "warm rebuild"
sep

# yatr (always present)
"$YATR" run build >/dev/null 2>&1 || true            # prime cache
read -r y_s_min y_s_mean < <(timeit "'$YATR' list")
read -r y_w_min y_w_mean < <(timeit "'$YATR' run build")
row "yatr" "$y_s_min / $y_s_mean" "$y_w_min / $y_w_mean"

if have make; then
  make build >/dev/null 2>&1 || true
  read -r m_s_min m_s_mean < <(timeit "make -n build")
  read -r m_w_min m_w_mean < <(timeit "make build")
  row "make" "$m_s_min / $m_s_mean" "$m_w_min / $m_w_mean"
fi

if have just; then
  just build >/dev/null 2>&1 || true
  read -r j_s_min j_s_mean < <(timeit "just --list")
  read -r j_w_min j_w_mean < <(timeit "just build")
  row "just (no caching)" "$j_s_min / $j_s_mean" "$j_w_min / $j_w_mean"
fi

if have task; then
  task build >/dev/null 2>&1 || true
  read -r t_s_min t_s_mean < <(timeit "task --list-all")
  read -r t_w_min t_w_mean < <(timeit "task build")
  row "task" "$t_s_min / $t_s_mean" "$t_w_min / $t_w_mean"
fi

echo
echo "Workload: a single 'build' task that does ~100ms of work and writes an"
echo "output from a source. 'warm rebuild' re-invokes build with nothing changed."
