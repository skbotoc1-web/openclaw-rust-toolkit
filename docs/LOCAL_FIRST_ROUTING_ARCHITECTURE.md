# Local-First Routing Architecture (Cost + Reliability)

## Ziel
Cloud-API nur nutzen, wenn lokal nicht ausreicht.

## 3-Block-Architektur

### 1) Request Router (Policy Engine)
Entscheidet pro Request:
- `local_only`
- `local_then_cloud`
- `cloud_only`

Input-Signale:
- Task-Klasse (`embed`, `classify`, `extract`, `summarize`, `reasoning`)
- Risiko/Qualität (confidence, schema-validierung)
- Budget (daily/monthly limits)
- Latenz-SLA
- Sicherheitskontext (PII/sensitive)

### 2) Local Inference Layer
- **Embeddings lokal** (RAG-Workloads)
- **Small Models lokal** für einfache Aufgaben
- Ergebnis mit Confidence + Quality Flags

### 3) Cloud Escalation Layer
- Eskalation nur bei Gates:
  - confidence < threshold
  - validation failed
  - hard task class
  - explicit cloud-required policy

---

## Default-Policy (v1)
1. `embed` => `local_only`
2. `classify|extract` => `local_then_cloud`
3. `summarize` => `local_then_cloud` (bei quality fail eskalieren)
4. `reasoning|high-risk` => `cloud_only`

Hard Stops:
- Budget hard limit erreicht => kein `cloud_only`, nur degradierter lokaler Modus
- Rate limit aktiv => Backoff + Queue + Retry-Budget

---

## KPI-Framework (No metric, no merge)
Pflichtmetriken pro PR:
- `local_hit_rate`
- `cloud_escalation_rate`
- `cache_hit_rate`
- `estimated_tokens_saved`
- `cost_per_1k_requests`
- `p95_latency`
- `quality_fail_rate`
- `missed_signal_incidents`

---

## PR-Slices (implementierbar)

### PR-1: Policy-Schema + Router Skeleton
- config: `policies/router.toml`
- task classification + route decision (ohne model execution)
- structured decision log

**Akzeptanzkriterien**
- deterministische Entscheidungen für identische Inputs
- unit tests für alle route outcomes

### PR-2: Local Embeddings Lane
- lokaler embedding-runner + health check
- local index write path (RAG)
- fallback reason codes

**Akzeptanzkriterien**
- `embed` requests laufen ohne Cloud
- benchmark report: cloud calls for embeddings = 0 in baseline suite

### PR-3: Small-Model Local Lane
- lokale inferenz für `classify/extract`
- confidence score + schema validation

**Akzeptanzkriterien**
- valid responses bei baseline tasks
- automatische escalation bei low confidence

### PR-4: Cloud Escalation + Budget Gates
- escalation adapter
- daily/monthly budget watchdog
- hard/soft limits + clear failure modes

**Akzeptanzkriterien**
- soft limit => warning event
- hard limit => cloud blocked + degradierter lokaler fallback

### PR-5: End-to-End Telemetry + Daily Report
- metrics emission (jsonl)
- daily KPI rollup + report artifact

**Akzeptanzkriterien**
- alle Pflichtmetriken vorhanden
- before/after report für token+cost

### PR-6: Marketing Proof Pack
- 3 reproduzierbare benchmark cases
- cost before/after sheet
- quality gate report (no missed critical signal)

**Akzeptanzkriterien**
- publish-ready assets in `docs/` + reusable snippets

---

## Risiko-Management
- Fail-open nur für nichtkritische Workflows
- High-risk flows fail-closed mit klaren Fehlercodes
- Rollback: Router kann global auf `cloud_only` oder `local_only` gestellt werden

---

## Positioning-Satz (extern)
"Cloud only when needed: deterministic local-first routing with measurable savings and safe escalation."
