# Cross-Platform Install (Linux, macOS, Windows)

## Linux / macOS (one line)

```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
```

## Windows PowerShell (one line)

```powershell
irm https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.ps1 | iex
```

## Release assets

Releases include prebuilt binaries for:
- `octk-x86_64-unknown-linux-gnu.tar.gz`
- `octk-aarch64-unknown-linux-gnu.tar.gz`
- `octk-x86_64-apple-darwin.tar.gz`
- `octk-aarch64-apple-darwin.tar.gz`
- `octk-x86_64-pc-windows-msvc.zip`

Each artifact has a matching checksum file: `SHA256SUMS-<target>.txt`.

## Verify

```bash
octk --help
```

or on PowerShell:

```powershell
octk --help
```
