# 14-Tage Ausführungsplan (Local-First)

## Woche 1

### Tag 1-2
PR-1: Router-Schema + deterministische Decision Engine
- Deliverables: `policies/router.toml`, Decision-Log, Unit-Tests

### Tag 3-4
PR-2: Local Embeddings Lane
- Deliverables: embedding runner interface, local-only baseline benchmark

### Tag 5-7
PR-3: Small-Model Lane (classify/extract)
- Deliverables: confidence scoring, schema validator, escalation trigger

## Woche 2

### Tag 8-9
PR-4: Cloud Escalation + Budget Gates
- Deliverables: soft/hard limit handling, retry budget, reason codes

### Tag 10-11
PR-5: Telemetry + KPI Daily Rollup
- Deliverables: jsonl events, KPI report generator, baseline dashboard md

### Tag 12-13
PR-6: Marketing Proof Pack
- Deliverables: 3 benchmark kits, before/after cost sheet, quality report

### Tag 14
Release Candidate
- `v0.3.0-rc1`
- full QA loop + release checklist + go/no-go

---

## Go/No-Go Kriterien
- Cloud-Calls für Embeddings in Baseline: **0**
- Lokale Trefferquote für `classify/extract`: **>=70%**
- Kostenreduktion ggü. cloud-only baseline: **>=40%**
- Kein kritischer Debug-Signalverlust
