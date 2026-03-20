$ErrorActionPreference = 'Stop'

$Repo = 'skbotoc1-web/openclaw-rust-toolkit'
$BinaryName = 'octk.exe'
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { Join-Path $HOME '.local\bin' }
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null

$arch = $env:PROCESSOR_ARCHITECTURE
$target = switch ($arch) {
  'AMD64' { 'x86_64-pc-windows-msvc' }
  default { throw "Unsupported Windows architecture: $arch" }
}

try {
  Write-Host "[install] Detect platform target: $target"
  $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
  $tag = $release.tag_name
  if (-not $tag) { throw 'Could not detect latest release tag' }

  $url = "https://github.com/$Repo/releases/download/$tag/octk-$target.zip"
  $tmp = Join-Path $env:TEMP ("octk-install-" + [guid]::NewGuid().ToString())
  New-Item -ItemType Directory -Path $tmp -Force | Out-Null

  $zipPath = Join-Path $tmp 'pkg.zip'
  Write-Host "[install] Download release: $tag"
  Invoke-WebRequest -Uri $url -OutFile $zipPath
  Expand-Archive -Path $zipPath -DestinationPath $tmp -Force

  $src = Join-Path $tmp 'octk.exe'
  if (-not (Test-Path $src)) { throw 'octk.exe not found in package' }

  Copy-Item $src (Join-Path $InstallDir 'octk.exe') -Force
  Write-Host "[install] Installed octk.exe to $InstallDir"
}
catch {
  throw "Release install failed: $($_.Exception.Message). Please build from source with cargo install --path ."
}

Write-Host "[install] done."
Write-Host "[install] If '$InstallDir' is not in PATH, add it first."
Write-Host 'Try: openclaw logs --limit 200 --plain | octk --mode auto --command "openclaw logs --limit 200 --plain" --emit-flag'
