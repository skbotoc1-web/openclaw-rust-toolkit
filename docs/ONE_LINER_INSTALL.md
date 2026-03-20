# One-liner Install + First Run

## Copy & paste (recommended)

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
```

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.ps1 | iex
```

This will:
1. Install `octk` from latest GitHub release asset
2. Fallback to source-build (Linux/macOS install script) if release install fails
3. Print a quick usage check command

## Inline prompt snippet (for chats/docs)

```text
Install optional OpenClaw Rust Toolkit layer:
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
Then run: openclaw logs --limit 200 --plain | octk --mode auto --command "openclaw logs --limit 200 --plain" --emit-flag
```
