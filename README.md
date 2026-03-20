# OpenClaw Rust Toolkit (MIT)

[![CI](https://github.com/skbotoc1-web/openclaw-rust-toolkit/actions/workflows/ci.yml/badge.svg)](https://github.com/skbotoc1-web/openclaw-rust-toolkit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Optional Rust Layer for OpenClaw command output: **activate only when useful**, keep signal lines, and always emit a **usage + benefit report**.

## Why this exists
Large command outputs (logs/find/diff/config dumps) burn tokens fast.
This toolkit adds a deterministic condensing step before LLM ingestion.

### What you get
- ✅ Optional activation (`auto|on|off`)
- ✅ Installation detection rule (`RUST_TOOLKIT_DETECTED`)
- ✅ Clear activation rules (`rules.toml`)
- ✅ Marker flag to see usage (`RUST_TOOLKIT_USED`)
- ✅ Benefit report (saved chars/tokens)
- ✅ Safety: signal lines are preserved (`error/warn/failed/...`)

---

## Mindest-Prerequisites

- OpenClaw CLI installiert (für OpenClaw-Use-Cases)
- Rust Toolchain nur für Source-Build (`rustc`, `cargo`), nicht nötig bei Release-Binary-Install
- Linux/macOS: `curl`, `tar`
- Windows: PowerShell 5+ und Internetzugriff auf GitHub Releases

## Local Compute & Memory (quick view)

Runtime (prebuilt binary):
- Minimum: 1 vCPU, 256 MB RAM, ~20 MB disk
- Recommended: 2+ vCPU, 512 MB RAM, ~100 MB disk

Source build (cargo):
- Minimum: 2 vCPU, 2 GB RAM, ~1.5 GB disk
- Recommended: 4+ vCPU, 4 GB RAM, ~3 GB disk

Full details: `docs/LOCAL_REQUIREMENTS.md`

## Quick Start

### One-liner install (copy & paste)

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

Value: schneller Check, ob der letzte Release-Deploy wirklich erfolgreich war.

### Wrapper controls

```bash
# default
OCTK_MODE=auto scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# force on/off
OCTK_MODE=on  scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
OCTK_MODE=off scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain

# require installation (fail fast if octk missing)
OCTK_REQUIRED=1 scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
```

### Manual build

```bash
cargo build --release
cat huge-output.txt | ./target/release/octk --mode auto --command "openclaw logs --plain"
```

### Klare Vorher/Nachher-Aussage (Token)

Beispiel-Statement (echte Messung):

```text
Ohne Rust Toolkit: 12,617 Tokens
Mit Rust Toolkit: 4,221 Tokens
Reduktion: 66.5%
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

You will see a marker like:

```text
[RUST_TOOLKIT_USED] used=true reason=always_match saved=70.2% chars:49704->14822 tokens~:12426->3705
```

---

## Activation Rules

Defined in `rules.example.toml`:

- `always_match`: commands that should always use condensing
- `never_match`: commands that must never be condensed
- `min_input_chars` / `min_input_lines`: threshold activation in `auto`
- `flag`: marker prefix in the usage report

See: [`docs/OPENCLAW_INTEGRATION.md`](docs/OPENCLAW_INTEGRATION.md)

---

## Community Traction Pack

This repo includes:
- Implementation spec (`docs/IMPLEMENTATION_SPEC.md`)
- OpenClaw integration rules (`docs/OPENCLAW_INTEGRATION.md`)
- One-liner install doc (`docs/ONE_LINER_INSTALL.md`)
- OpenAI token benchmark guide (`docs/OPENAI_TOKEN_BENCHMARK.md`)
- Clear before/after token examples (`docs/CLEAR_METRICS_EXAMPLES.md`)
- Wrapper script for instant trials (`scripts/openclaw-wrap.sh`)
- Real host benchmark (`docs/BENCHMARK_2026-03-20.md`)
- Community rollout guide (`docs/COMMUNITY_PLAYBOOK.md`)
- Cross-platform install guide (`docs/CROSS_PLATFORM_INSTALL.md`)
- Upgrade-safe integration strategy (`docs/UPGRADE_SAFE_STRATEGY.md`)
- Compatibility matrix (`docs/COMPATIBILITY.md`)
- Security policy + secret hygiene (`SECURITY.md`, `scripts/secret-check.sh`)

If you run this in production, open an issue with:
1. command classes
2. avg token savings
3. any missed-debug incidents

---

## License

MIT. See [LICENSE](LICENSE).

Copyright: Stefan Kaiser — https://stefankaiser.net
 an issue with:
1. command classes
2. avg token savings
3. any missed-debug incidents

---

## License

MIT. See [LICENSE](LICENSE).

Copyright: Stefan Kaiser — https://stefankaiser.net
