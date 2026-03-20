$ErrorActionPreference = 'Stop'

param(
  [string]$Repo = "skbotoc1-web/openclaw-rust-toolkit",
  [switch]$RequireSuccess,
  [int]$MinAssets = 0,
  [int]$MaxAgeHours = 0
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

$assetCount = 0
if ($rel) {
  if ($rel.assets) { $assetCount = $rel.assets.Count }
  Write-Output "[LAST_DEPLOY_STATUS] release_tag=$($rel.tagName) draft=$($rel.isDraft) prerelease=$($rel.isPrerelease) assets=$assetCount"
  Write-Output "[LAST_DEPLOY_STATUS] release_url=$($rel.url) published=$($rel.publishedAt)"
} else {
  Write-Output "[LAST_DEPLOY_STATUS] release_tag=none"
}

if ($RequireSuccess) {
  $errors = @()

  if (-not $run) {
    $errors += 'no_release_workflow_run'
  } elseif ($run.status -ne 'completed' -or $run.conclusion -ne 'success') {
    $errors += 'latest_release_run_not_success'
  }

  if (-not $rel) {
    $errors += 'no_release_metadata'
  } else {
    if ($MinAssets -gt 0 -and $assetCount -lt $MinAssets) {
      $errors += "assets_below_min:${assetCount}<${MinAssets}"
    }

    if ($MaxAgeHours -gt 0) {
      try {
        $published = [datetimeoffset]::Parse($rel.publishedAt)
        $ageHours = ((Get-Date).ToUniversalTime() - $published.UtcDateTime).TotalHours
        if ($ageHours -gt $MaxAgeHours) {
          $errors += "release_too_old:$([math]::Round($ageHours,1))h>$MaxAgeHours`h"
        }
      } catch {
        $errors += 'release_age_parse_failed'
      }
    }
  }

  if ($errors.Count -gt 0) {
    Write-Output "[LAST_DEPLOY_STATUS] policy_ok=false errors=$($errors -join ',')"
    exit 3
  } else {
    Write-Output "[LAST_DEPLOY_STATUS] policy_ok=true"
  }
}
