# Launch Post (EN)

## Hook
We built a tiny Rust layer for OpenClaw that reduces noisy CLI context **by ~70–85%** before it hits your LLM.

## Problem
Agent workflows waste tokens on long logs/diffs/search output.
That increases cost, latency, and context pressure.

## Solution
**OpenClaw Rust Toolkit (`octk`)**
- deterministic condensing (not black-box summarization)
- profile presets (`safe`, `balanced`, `aggressive`)
- explicit usage markers + measurable token savings
- local-first router dry-run (policy-based route decisions)
- local embeddings lane (`route=local_only`, `cloud_calls=0`)
- fail-open design (no OpenClaw core patching)

## Real numbers
From real host runs:
- 12,970 -> 2,929 tokens (**-77.4%**, balanced)
- 12,970 -> 2,032 tokens (**-84.3%**, aggressive)

## Try in 60 seconds
```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
OCTK_PROFILE=balanced scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
```

## Repo
https://github.com/skbotoc1-web/openclaw-rust-toolkit

## Ask
Would love feedback on:
1. your real-world token savings
2. local-first routing thresholds (confidence/budget gates)
3. missed-debug edge cases
