# Klare Vorher/Nachher-Metriken (Token-Reduktion)

## Statement 1 — OpenClaw Logs

**Ohne Rust Toolkit:**
- Input: `50,471` chars (~`12,617` tokens)

**Mit Rust Toolkit (`octk --mode on`):**
- Output: `16,887` chars (~`4,221` tokens)

**Reduktion:**
- `8,396` Tokens weniger
- `66.5%` weniger Token

Formel:
`(12617 - 4221) / 12617 = 66.5%`

Quelle: `.reports/example-logs-400.json`

---

## Statement 2 — Pfadscan (großer Output)

**Ohne Rust Toolkit:**
- Input: `6,648,250` chars (~`1,662,062` tokens)

**Mit Rust Toolkit (`octk --mode on`):**
- Output: `197,818` chars (~`49,454` tokens)

**Reduktion:**
- `1,612,608` Tokens weniger
- `97.0%` weniger Token

Formel:
`(1662062 - 49454) / 1662062 = 97.0%`

Quelle: `.reports/example-root-scan.json`

---

## Standardisierte Reporting-Vorlage

```text
Statement <n>: <command/use-case>
Ohne Rust Toolkit: <x> tokens
Mit Rust Toolkit: <y> tokens
Reduktion: <z>%
```
