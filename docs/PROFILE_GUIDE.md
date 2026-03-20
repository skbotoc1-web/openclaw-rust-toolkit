# Profile Guide

## Goal
Profiles let users pick a token-vs-context tradeoff without editing TOML manually.

## Profiles

### safe
- Highest context retention
- Best when debugging sensitive/complex incidents
- Lowest compression of the 3 presets

### balanced (default)
- Good default for day-to-day OpenClaw usage
- Solid reduction while preserving key signal lines

### aggressive
- Maximum reduction focus
- Useful for very noisy outputs and cost-sensitive workloads
- May require fallback to `safe`/`off` for deep debugging

## Usage

```bash
OCTK_PROFILE=safe scripts/openclaw-wrap.sh -- openclaw logs --limit 300 --plain
OCTK_PROFILE=balanced scripts/openclaw-wrap.sh -- openclaw logs --limit 300 --plain
OCTK_PROFILE=aggressive scripts/openclaw-wrap.sh -- openclaw logs --limit 300 --plain
```

## Override profile with custom rules

```bash
OCTK_RULES=./rules.example.toml scripts/openclaw-wrap.sh -- openclaw logs --limit 300 --plain
```
