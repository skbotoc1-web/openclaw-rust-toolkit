# Community Playbook: Reduce OpenClaw Token Pain Together

## Goal
Create a shared, reproducible baseline for token reduction without killing debug quality.

## What the community can contribute
1. Real-world benchmark reports
2. Better activation heuristics per command class
3. Safe defaults for different environments (laptop, homelab, VPS)
4. Regression examples where condensing hid critical detail

## Proposed shared benchmark format
- command class
- raw chars/tokens
- condensed chars/tokens
- savings %
- fix-rate impact (if measured)
- missed-signal incidents

## High-value roadmap
- Rule profiles (`logs`, `search`, `json`, `diff`)
- Confidence score for "safe to condense"
- Optional semantic fallback for dense JSON (without losing keys)
- OpenClaw plugin wrapper example with daily KPI summary

## Publishing guideline
Never publish:
- `.env` content
- auth headers/tokens
- full private logs containing IDs/emails/secrets

Always publish:
- anonymized metrics
- command class + volume
- quality outcomes
