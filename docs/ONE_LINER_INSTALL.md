# One-liner Install + First Run

## Copy & paste (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
```

This will:
1. Install Rust if missing
2. Install `octk`
3. Print a quick usage check command

## Inline prompt snippet (for chats/docs)

```text
Install optional OpenClaw Rust Toolkit layer:
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
Then run: openclaw logs --limit 200 --plain | octk --mode auto --command "openclaw logs --limit 200 --plain" --emit-flag
```
