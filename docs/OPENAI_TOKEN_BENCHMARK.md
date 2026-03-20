# OpenAI Token Reduction Benchmark (practical)

## Goal
Measure how much `octk` reduces effective prompt payload before OpenAI model calls.

## KPI set
- Input chars/raw
- Input chars/condensed
- Estimated tokens raw (`chars/4`)
- Estimated tokens condensed (`chars/4`)
- Savings %
- Quality check: did we lose critical debugging signal?

## Quick benchmark command

```bash
openclaw logs --limit 400 --plain > /tmp/raw.log
cat /tmp/raw.log | octk --mode auto --command "openclaw logs --limit 400 --plain" --report-file /tmp/octk-report.json --emit-flag > /tmp/condensed.log
cat /tmp/octk-report.json
```

## How to present results (community friendly)
Use this table format in README/issues:

| Case | Raw chars | Condensed chars | Raw tokens~ | Condensed tokens~ | Savings |
|---|---:|---:|---:|---:|---:|
| openclaw logs --limit 400 | 49,704 | 14,822 | 12,426 | 3,705 | 70.2% |

## OpenAI-focused framing
When sharing results, map directly to cost/rate impact:
- Fewer prompt tokens => lower cost per request
- Lower context pressure => fewer truncation risks
- Smaller payload => often faster model latency

## Quality gate (must-have)
For each benchmark, verify:
1. All `error/warn/failed/timeout/...` lines still present
2. Root-cause line still visible
3. If not: run with `--mode off` (full output)
