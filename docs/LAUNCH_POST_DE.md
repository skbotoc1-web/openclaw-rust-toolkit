# Launch Post (DE)

## Hook
Wir haben ein kleines Rust-Layer für OpenClaw gebaut, das noisy CLI-Output vor dem LLM um **~70–85%** reduziert.

## Problem
Agent-Workflows verbrennen viele Tokens auf langen Logs/Diffs/Search-Outputs.
Das erhöht Kosten, Latenz und drückt auf das Context Window.

## Lösung
**OpenClaw Rust Toolkit (`octk`)**
- deterministisches Condensing (kein Black-Box-Summarizing)
- Profile (`safe`, `balanced`, `aggressive`)
- klare Marker + messbare Token-Einsparung
- Local-First Router Dry-Run (policy-basierte Routenentscheidung)
- fail-open Design (kein OpenClaw-Core-Patching)

## Reale Zahlen
Aus echten Host-Runs:
- 12.970 -> 2.929 Tokens (**-77,4%**, balanced)
- 12.970 -> 2.032 Tokens (**-84,3%**, aggressive)

## In 60 Sekunden testen
```bash
curl -fsSL https://raw.githubusercontent.com/skbotoc1-web/openclaw-rust-toolkit/main/scripts/install.sh | bash
OCTK_PROFILE=balanced scripts/openclaw-wrap.sh -- openclaw logs --limit 200 --plain
```

## Repo
https://github.com/skbotoc1-web/openclaw-rust-toolkit

## Feedback gesucht
1. eure realen Savings
2. sinnvolle Local-First Schwellen (Confidence/Budget-Gates)
3. edge cases bei Debug-Qualität
