#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/openclaw-wrap.sh -- openclaw logs --limit 400 --plain
#
# Behavior:
# - Executes command
# - Detects whether Rust Toolkit is installed
# - Applies optional condensing layer when available
# - Emits usage marker/report to stderr
# - Preserves original exit code
#
# Env controls:
#   OCTK_MODE=auto|on|off                 (default: auto)
#   OCTK_REQUIRED=1                       (fail if toolkit is not installed)
#   OCTK_PROFILE=safe|balanced|aggressive (default: balanced)
#   OCTK_RULES=/path/to/rules.toml        (overrides profile)
#   OCTK_LOG_LEVEL=error|warn|info|debug  (default: warn)
#   OCTK_DEBUG=1                          (forces debug logging)
# Compatible aliases: OPENCLAW_LOG_LEVEL / OPENCLAW_DEBUG

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
OCTK_MODE="${OCTK_MODE:-auto}"
OCTK_REQUIRED="${OCTK_REQUIRED:-0}"
OCTK_PROFILE="${OCTK_PROFILE:-balanced}"
OCTK_RULES="${OCTK_RULES:-}"

if [[ -z "$OCTK_RULES" ]]; then
  case "$OCTK_PROFILE" in
    safe|balanced|aggressive)
      OCTK_RULES="./profiles/${OCTK_PROFILE}.toml"
      ;;
    *)
      echo "[RUST_TOOLKIT_ERROR] invalid OCTK_PROFILE=$OCTK_PROFILE (expected safe|balanced|aggressive)" >&2
      rm -f "$TMP_OUT" "$TMP_REPORT"
      exit 43
      ;;
  esac
fi

if [[ ! -f "$OCTK_RULES" ]]; then
  echo "[RUST_TOOLKIT_ERROR] rules file not found: $OCTK_RULES" >&2
  rm -f "$TMP_OUT" "$TMP_REPORT"
  exit 44
fi

set +e
"${CMD[@]}" >"$TMP_OUT" 2>&1
STATUS=$?
set -e

OCTK_BIN=""
if command -v octk >/dev/null 2>&1; then
  OCTK_BIN="$(command -v octk)"
elif [[ -x "./target/release/octk" ]]; then
  OCTK_BIN="./target/release/octk"
fi

if [[ -n "$OCTK_BIN" ]]; then
  OCTK_VERSION="$($OCTK_BIN --version 2>/dev/null || echo unknown)"
  echo "[RUST_TOOLKIT_DETECTED] installed=true bin=$OCTK_BIN version=$OCTK_VERSION mode=$OCTK_MODE profile=$OCTK_PROFILE rules=$OCTK_RULES" >&2

  cat "$TMP_OUT" | "$OCTK_BIN" \
    --mode "$OCTK_MODE" \
    --command "${CMD[*]}" \
    --rules "$OCTK_RULES" \
    --report-format text \
    --report-file "$TMP_REPORT" \
    --emit-flag
else
  cat "$TMP_OUT"
  echo "[RUST_TOOLKIT_DETECTED] installed=false mode=$OCTK_MODE profile=$OCTK_PROFILE rules=$OCTK_RULES" >&2
  echo "[RUST_TOOLKIT_USED] used=false reason=octk_not_installed saved=0.0%" >&2

  if [[ "$OCTK_REQUIRED" == "1" ]]; then
    echo "[RUST_TOOLKIT_ERROR] OCTK_REQUIRED=1 but toolkit is not installed" >&2
    rm -f "$TMP_OUT" "$TMP_REPORT"
    exit 42
  fi
fi

# Optional: keep JSON report artifact
if [[ -s "$TMP_REPORT" ]]; then
  mkdir -p .reports
  cp "$TMP_REPORT" ".reports/$(date +%F-%H%M%S)-octk-report.json"
fi

rm -f "$TMP_OUT" "$TMP_REPORT"
exit "$STATUS"
