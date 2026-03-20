# Clear Before/After Metrics (Token Reduction)

## Statement 1 — OpenClaw Logs

**Without Rust Toolkit:**
- Input: `50,471` chars (~`12,617` tokens)

**With Rust Toolkit (`octk --mode on`):**
- Output: `16,887` chars (~`4,221` tokens)

**Reduction:**
- `8,396` fewer tokens
- `66.5%` token reduction

Formula:
`(12617 - 4221) / 12617 = 66.5%`

Source: `.reports/example-logs-400.json`

---

## Statement 2 — Path Scan (Large Output)

**Without Rust Toolkit:**
- Input: `6,648,250` chars (~`1,662,062` tokens)

**With Rust Toolkit (`octk --mode on`):**
- Output: `197,818` chars (~`49,454` tokens)

**Reduction:**
- `1,612,608` fewer tokens
- `97.0%` token reduction

Formula:
`(1662062 - 49454) / 1662062 = 97.0%`

Source: `.reports/example-root-scan.json`

---

## Standard Reporting Template

```text
Statement <n>: <command/use-case>
Without Rust Toolkit: <x> tokens
With Rust Toolkit: <y> tokens
Reduction: <z>%
```
