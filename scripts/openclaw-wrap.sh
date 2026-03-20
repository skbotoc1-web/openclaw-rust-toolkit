#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/openclaw-wrap.sh -- openclaw logs --limit 400 --plain
#
# Behavior:
# - Executes command
# - Pipes combined output through octk condense (optional layer)
# - Emits usage marker/report to stderr
# - Preserves original exit code

if [[ "${1:-}" == "--" ]]; then
  shift
fi

if [[ $# -eq 0 ]]; then
  echo "usage: $0 -- <command...>" >&2
  exit 2
fi

CMD=("$@")
TMP_OUT=$(mktemp)
TMP_REPORT=$(mktemp)

set +e
"${CMD[@]}" >"$TMP_OUT" 2>&1
STATUS=$?
set -e

OCTK_BIN=""
if command -v octk >/dev/null 2>&1; then
  OCTK_BIN="$(command -v octk)"
elif [[ -x "./target/release/openclaw-rust-toolkit" ]]; then
  OCTK_BIN="./target/release/openclaw-rust-toolkit"
fi

if [[ -n "$OCTK_BIN" ]]; then
  cat "$TMP_OUT" | "$OCTK_BIN" \
    --mode auto \
    --command "${CMD[*]}" \
    --rules ./rules.example.toml \
    --report-format text \
    --report-file "$TMP_REPORT" \
    --emit-flag
else
  cat "$TMP_OUT"
  echo "[RUST_TOOLKIT_USED] used=false reason=octk_not_installed saved=0.0%" >&2
fi

# Optional: keep JSON report artifact
if [[ -s "$TMP_REPORT" ]]; then
  mkdir -p .reports
  cp "$TMP_REPORT" ".reports/$(date +%F-%H%M%S)-octk-report.json"
fi

rm -f "$TMP_OUT" "$TMP_REPORT"
exit "$STATUS"
