# Last Deploy Status

Use this to quickly verify whether the latest release deployment is healthy.

## Linux/macOS

```bash
scripts/last-deploy-status.sh
# or for another repo
scripts/last-deploy-status.sh owner/repo
```

## Windows PowerShell

```powershell
./scripts/last-deploy-status.ps1
# or for another repo
./scripts/last-deploy-status.ps1 -Repo owner/repo
```

## Example output

```text
[LAST_DEPLOY_STATUS] repo=skbotoc1-web/openclaw-rust-toolkit run_status=completed conclusion=success run_id=23341903977
[LAST_DEPLOY_STATUS] run_url=https://github.com/... created=... updated=...
[LAST_DEPLOY_STATUS] release_tag=v0.2.1 draft=False prerelease=False assets=4
[LAST_DEPLOY_STATUS] release_url=https://github.com/... published=...
```

## Value for users
- Fast health check after each release
- One line proof for CI/deploy status in support/debug threads
- Easy integration into cron/heartbeat checks
