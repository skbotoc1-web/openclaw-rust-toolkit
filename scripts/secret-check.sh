#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "[secret-check] scanning tracked files for risky patterns..."
rg -n "ghp_|api[_-]?key|token\s*=|secret\s*=|BEGIN [A-Z ]*PRIVATE KEY|\.env" -S . \
  --glob '!target/**' \
  --glob '!Cargo.lock' || true

echo "[secret-check] done. Review matches manually before publish."
