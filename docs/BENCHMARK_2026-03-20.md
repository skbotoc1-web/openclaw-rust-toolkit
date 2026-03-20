# Benchmark 2026-03-20 (Host sample)

| Case | Raw chars | Condensed chars | Savings |
|---|---:|---:|---:|
| openclaw logs | 49,704 | 14,822 | 70.2% |
| path scan `/root/.openclaw` | 6,601,256 | 602,278 | 90.9% |
| docs inventory | 44,250 | 3,967 | 91.0% |
| openclaw status --json | 20,000 | 2,170 | 89.1% |

Average savings: **85.3%**

> Important: Higher compression can hide edge-case details. Keep fallback to raw output for deep debugging.
