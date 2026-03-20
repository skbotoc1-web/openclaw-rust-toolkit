#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/last-deploy-status.sh [owner/repo]
#   scripts/last-deploy-status.sh --repo owner/repo --require-success --min-assets 3 --max-age-hours 168
#
# Exit codes:
#   0  ok
#   2  gh not found
#   3  policy check failed (when --require-success is enabled)

REPO="${REPO:-skbotoc1-web/openclaw-rust-toolkit}"
REQUIRE_SUCCESS=0
MIN_ASSETS=0
MAX_AGE_HOURS=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      REPO="$2"; shift 2 ;;
    --require-success)
      REQUIRE_SUCCESS=1; shift ;;
    --min-assets)
      MIN_ASSETS="$2"; shift 2 ;;
    --max-age-hours)
      MAX_AGE_HOURS="$2"; shift 2 ;;
    *)
      # Backward compatible positional repo arg
      REPO="$1"; shift ;;
  esac
done

if ! command -v gh >/dev/null 2>&1; then
  echo "[LAST_DEPLOY_STATUS] error=gh_cli_not_found repo=$REPO"
  exit 2
fi

RUN_JSON="$(gh run list --repo "$REPO" --workflow Release --limit 1 --json databaseId,status,conclusion,createdAt,updatedAt,url,headBranch,displayTitle 2>/dev/null || true)"
REL_JSON="$(gh release view --repo "$REPO" --json tagName,publishedAt,isDraft,isPrerelease,url,assets,name 2>/dev/null || true)"

python3 - <<'PY' "$REPO" "$RUN_JSON" "$REL_JSON" "$REQUIRE_SUCCESS" "$MIN_ASSETS" "$MAX_AGE_HOURS"
import datetime as dt
import json
import sys

repo, run_json, rel_json, require_success, min_assets, max_age_hours = sys.argv[1:7]
require_success = bool(int(require_success))
min_assets = int(min_assets)
max_age_hours = int(max_age_hours)

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

assets_count = 0
if rel:
    assets = rel.get('assets') or []
    assets_count = len(assets)
    print(f"[LAST_DEPLOY_STATUS] release_tag={rel.get('tagName')} draft={rel.get('isDraft')} prerelease={rel.get('isPrerelease')} assets={assets_count}")
    print(f"[LAST_DEPLOY_STATUS] release_url={rel.get('url')} published={rel.get('publishedAt')}")
else:
    print(f"[LAST_DEPLOY_STATUS] release_tag=none")

if require_success:
    errors = []
    if not run:
        errors.append("no_release_workflow_run")
    else:
        if run.get("status") != "completed" or run.get("conclusion") != "success":
            errors.append("latest_release_run_not_success")

    if not rel:
        errors.append("no_release_metadata")
    else:
        if min_assets > 0 and assets_count < min_assets:
            errors.append(f"assets_below_min:{assets_count}<{min_assets}")

        if max_age_hours > 0:
            try:
                published = dt.datetime.fromisoformat(rel.get("publishedAt").replace("Z", "+00:00"))
                now = dt.datetime.now(dt.timezone.utc)
                age_h = (now - published).total_seconds() / 3600
                if age_h > max_age_hours:
                    errors.append(f"release_too_old:{age_h:.1f}h>{max_age_hours}h")
            except Exception:
                errors.append("release_age_parse_failed")

    if errors:
        print(f"[LAST_DEPLOY_STATUS] policy_ok=false errors={','.join(errors)}")
        sys.exit(3)
    else:
        print("[LAST_DEPLOY_STATUS] policy_ok=true")
PY
