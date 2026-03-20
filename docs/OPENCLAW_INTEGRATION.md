# OpenClaw Integration Guide

## Objective
Use Rust Toolkit as an **optional pre-LLM layer** for high-volume outputs, with explicit signaling and measurable benefit.

## Integration Modes

### 1) Auto (recommended)
- Use toolkit when command is in `always_match` OR output crosses thresholds.
- Skip when command is in `never_match`.

### 2) On
- Force toolkit for all outputs.
- Useful in cost-sensitive runs.

### 3) Off
- Disable toolkit entirely.
- Useful for full-fidelity debugging.

## Activation Rules (required)

1. Always ON for:
   - `openclaw logs`
   - `docker logs`
   - `git diff`
   - `find`
   - `rg`
2. Never ON for:
   - commands touching secrets/private keys
3. Threshold ON if either:
   - input chars >= 4000
   - input lines >= 120

## Visibility Contract (required)
Every run must emit a marker flag line to stderr:

```text
[RUST_TOOLKIT_USED] used=true|false reason=<...> saved=<...>% chars:<in>-><out> tokens~:<in>-><out>
```

## Benefit Report Contract (required)
Write JSON report artifact per run with at least:
- used
- reason
- input_chars/output_chars
- estimated_input_tokens/estimated_output_tokens
- saved_percent
- command
- timestamp (wrapper adds this)

## Suggested OpenClaw Hook Pattern
- Wrap heavy commands via `scripts/openclaw-wrap.sh`
- Store reports under `.reports/`
- Summarize daily with simple script/cron

## Guardrails
- Preserve signal lines (`error|warn|failed|...`)
- Keep fallback path to full raw output
- Do not condense security-sensitive outputs by default
