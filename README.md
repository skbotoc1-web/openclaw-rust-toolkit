# OpenClaw Rust Toolkit (MIT)

[![CI](https://github.com/skbotoc1-web/openclaw-rust-toolkit/actions/workflows/ci.yml/badge.svg)](https://github.com/skbotoc1-web/openclaw-rust-toolkit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Optional Rust layer for OpenClaw command output: **activate only when useful**, preserve signal lines, and emit a **usage + savings report**.

## Quick value (what you get in practice)
- Real-world reduction: **~70–85% fewer prompt tokens** on noisy CLI output
- Deterministic behavior (rules), not black-box summarization
- Works as an optional external layer (upgrade-safe)

## 60-second verification
```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
OCTK_PROFILE=balanced scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
```

Look for:
- `[RUST_TOOLKIT_DETECTED] ...`
- `[RUST_TOOLKIT_USED] ... saved=...%`

## Why this exists
Large CLI outputs (logs/find/diff/config dumps) consume tokens quickly.
This toolkit adds a deterministic condensing step before LLM ingestion.

## What you get
- ✅ Optional activation (`auto|on|off`)
- ✅ Installation detection marker (`RUST_TOOLKIT_DETECTED`)
- ✅ Clear activation rules (`rules.example.toml`)
- ✅ Usage marker (`RUST_TOOLKIT_USED`)
- ✅ Benefit report (saved chars/tokens)
- ✅ Safety: signal lines are preserved (`error|warn|failed|...`)

---

## Minimum prerequisites
- OpenClaw CLI (for OpenClaw-related workflows)
- Rust toolchain only for source build (`rustc`, `cargo`) — not needed for prebuilt release binaries
- Linux/macOS: `curl`, `tar`
- Windows: PowerShell 5+ and access to GitHub releases

## Local compute & memory (quick view)
Runtime (prebuilt binary):
- Minimum: 1 vCPU, 256 MB RAM, ~20 MB disk
- Recommended: 2+ vCPU, 512 MB RAM, ~100 MB disk

Source build (`cargo`):
- Minimum: 2 vCPU, 2 GB RAM, ~1.5 GB disk
- Recommended: 4+ vCPU, 4 GB RAM, ~3 GB disk

Full details: `docs/LOCAL_REQUIREMENTS.md`

---

## Quick start

### One-line install
Linux/macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
```

Windows PowerShell:
```powershell
irm https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.ps1 | iex
```

### Last deploy status (post-release health)
```bash
scripts/last-deploy-status.sh
```

```powershell
./scripts/last-deploy-status.ps1
```

### Enforced deploy health check (automation mode)
```bash
scripts/last-deploy-status.sh --require-success --min-assets 3 --max-age-hours 720
```

### Release integrity
Each release publishes per-target checksum files (`SHA256SUMS-<target>.txt`) alongside binaries.

### Wrapper controls
```bash
# default (balanced profile)
OCTK_MODE=auto scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# profile presets
OCTK_PROFILE=safe       scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
OCTK_PROFILE=balanced   scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
OCTK_PROFILE=aggressive scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# force on/off
OCTK_MODE=on  scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
OCTK_MODE=off scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# custom rules file (overrides profile)
OCTK_RULES=./rules.example.toml scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# fail if toolkit is not installed
OCTK_REQUIRED=1 scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
```

### Manual build
```bash
cargo build --release
cat huge-output.txt | ./target/release/octk --mode auto --command "openclaw logs --plain"
```

### Clear before/after token statement
```text
Without Rust Toolkit: 12,617 tokens
With Rust Toolkit: 4,221 tokens
Reduction: 66.5%
```

Details: `docs/CLEAR_METRICS_EXAMPLES.md`

### With report and custom rules
```bash
cat huge-output.txt | ./target/release/octk \
  --mode auto \
  --command "openclaw logs --limit 400 --plain" \
  --rules ./rules.example.toml \
  --report-format text \
  --report-file ./.reports/example.json \
  --emit-flag
```

Example markers:
```text
[RUST_TOOLKIT_DETECTED] installed=true bin=/home/user/.cargo/bin/octk version=octk 0.1.0 mode=auto
[RUST_TOOLKIT_USED] used=true reason=always_match saved=70.2% chars:49704->14822 tokens~:12426->3705
```

If not installed:
```text
[RUST_TOOLKIT_DETECTED] installed=false mode=auto
[RUST_TOOLKIT_USED] used=false reason=octk_not_installed saved=0.0%
```

---

## Activation rules
Defined in `rules.example.toml`:
- `always_match`: commands that should always use condensing
- `never_match`: commands that must never be condensed
- `min_input_chars` / `min_input_lines`: threshold activation in `auto`
- `flag`: marker prefix in the usage report

See: `docs/OPENCLAW_INTEGRATION.md`

---

## Included docs and assets
- `docs/IMPLEMENTATION_SPEC.md`
- `docs/OPENCLAW_INTEGRATION.md`
- `docs/PROFILE_GUIDE.md`
- `docs/ONE_LINER_INSTALL.md`
- `docs/CROSS_PLATFORM_INSTALL.md`
- `docs/UPGRADE_SAFE_STRATEGY.md`
- `docs/COMPATIBILITY.md`
- `docs/LOCAL_REQUIREMENTS.md`
- `docs/OPENAI_TOKEN_BENCHMARK.md`
- `docs/CLEAR_METRICS_EXAMPLES.md`
- `docs/LAST_DEPLOY_STATUS.md`
- `docs/BENCHMARK_2026-03-20.md`
- `docs/COMMUNITY_PLAYBOOK.md`
- `docs/LAUNCH_POST_EN.md`, `docs/LAUNCH_POST_DE.md`
- `docs/MOAT_POSITIONING.md`
- `docs/ENTERPRISE_READINESS.md`
- `docs/PRODUCT_GUARDRAILS.md`
- `SECURITY.md`, `scripts/secret-check.sh`

If you run this in production, open an issue with:
1. command classes
2. average token savings
3. missed-debug incidents (if any)

---

## License
MIT. See [LICENSE](LICENSE).

Copyright: Stefan Kaiser — https://stefankaiser.net
