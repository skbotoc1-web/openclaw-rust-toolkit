$ErrorActionPreference = 'Stop'

param(
  [string]$Repo = "skbotoc1-web/openclaw-rust-toolkit"
)

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
  Write-Output "[LAST_DEPLOY_STATUS] error=gh_cli_not_found repo=$Repo"
  exit 2
}

$runJson = gh run list --repo $Repo --workflow Release --limit 1 --json databaseId,status,conclusion,createdAt,updatedAt,url,headBranch,displayTitle 2>$null
$relJson = gh release view --repo $Repo --json tagName,publishedAt,isDraft,isPrerelease,url,assets,name 2>$null

$run = $null
if ($runJson) {
  $runs = $runJson | ConvertFrom-Json
  if ($runs.Count -gt 0) { $run = $runs[0] }
}

$rel = $null
if ($relJson) {
  $rel = $relJson | ConvertFrom-Json
}

if ($run) {
  Write-Output "[LAST_DEPLOY_STATUS] repo=$Repo run_status=$($run.status) conclusion=$($run.conclusion) run_id=$($run.databaseId)"
  Write-Output "[LAST_DEPLOY_STATUS] run_url=$($run.url) created=$($run.createdAt) updated=$($run.updatedAt)"
} else {
  Write-Output "[LAST_DEPLOY_STATUS] repo=$Repo run_status=unknown conclusion=unknown"
}

if ($rel) {
  $assetCount = 0
  if ($rel.assets) { $assetCount = $rel.assets.Count }
  Write-Output "[LAST_DEPLOY_STATUS] release_tag=$($rel.tagName) draft=$($rel.isDraft) prerelease=$($rel.isPrerelease) assets=$assetCount"
  Write-Output "[LAST_DEPLOY_STATUS] release_url=$($rel.url) published=$($rel.publishedAt)"
} else {
  Write-Output "[LAST_DEPLOY_STATUS] release_tag=none"
}
