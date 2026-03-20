#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/last-deploy-status.sh [owner/repo]
#   REPO=owner/repo scripts/last-deploy-status.sh

REPO_ARG="${1:-}"
REPO="${REPO_ARG:-${REPO:-skbotoc1-web/openclaw-rust-toolkit}}"

if ! command -v gh >/dev/null 2>&1; then
  echo "[LAST_DEPLOY_STATUS] error=gh_cli_not_found repo=$REPO"
  exit 2
fi

RUN_JSON="$(gh run list --repo "$REPO" --workflow Release --limit 1 --json databaseId,status,conclusion,createdAt,updatedAt,url,headBranch,displayTitle 2>/dev/null || true)"
REL_JSON="$(gh release view --repo "$REPO" --json tagName,publishedAt,isDraft,isPrerelease,url,assets,name 2>/dev/null || true)"

python3 - <<'PY' "$REPO" "$RUN_JSON" "$REL_JSON"
import json,sys
repo, run_json, rel_json = sys.argv[1], sys.argv[2], sys.argv[3]

run = None
try:
    arr = json.loads(run_json) if run_json.strip() else []
    run = arr[0] if arr else None
except Exception:
    run = None

rel = None
try:
    rel = json.loads(rel_json) if rel_json.strip() else None
except Exception:
    rel = None

if run:
    print(f"[LAST_DEPLOY_STATUS] repo={repo} run_status={run.get('status')} conclusion={run.get('conclusion')} run_id={run.get('databaseId')}")
    print(f"[LAST_DEPLOY_STATUS] run_url={run.get('url')} created={run.get('createdAt')} updated={run.get('updatedAt')}")
else:
    print(f"[LAST_DEPLOY_STATUS] repo={repo} run_status=unknown conclusion=unknown")

if rel:
    assets = rel.get('assets') or []
    print(f"[LAST_DEPLOY_STATUS] release_tag={rel.get('tagName')} draft={rel.get('isDraft')} prerelease={rel.get('isPrerelease')} assets={len(assets)}")
    print(f"[LAST_DEPLOY_STATUS] release_url={rel.get('url')} published={rel.get('publishedAt')}")
else:
    print(f"[LAST_DEPLOY_STATUS] release_tag=none")
PY
