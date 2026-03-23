# QA + Validation Report — Local-First Router PR1 (2026-03-23)

## Scope
- Router policy schema + deterministic decision engine
- CLI router dry-run mode + structured decision log
- Policy file baseline (`policies/router.toml`)

## Test/Validation Matrix

### A) Testen
Commands:
```bash
cargo test -q
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Result:
- `cargo test`: **22/22 passed**
- `cargo fmt --check`: **pass**
- `cargo clippy -D warnings`: **pass**

### B) Reparieren
- Keine Defekte nach Testlauf.
- Nur rustfmt-Layout-Anpassungen wurden automatisch angewendet.

### C) Validieren (behavioral)
Dry-run checks:
```bash
cargo run --quiet -- --route-task embed --route-policy ./policies/router.toml
cargo run --quiet -- --route-task reasoning --route-policy ./policies/router.toml
cargo run --quiet -- --route-task classify --route-confidence 0.90 --route-schema-valid true --route-policy ./policies/router.toml
cargo run --quiet -- --route-task classify --route-confidence 0.20 --route-schema-valid true --route-policy ./policies/router.toml
cargo run --quiet -- --route-task high_risk --route-budget-hard-limit --route-policy ./policies/router.toml
```

Observed routes:
- `embed` -> `local_only`
- `reasoning` -> `cloud_only`
- `classify(0.90)` -> `local_only`
- `classify(0.20)` -> `local_then_cloud`
- `high_risk + hard_budget` -> `local_only` (degraded local)

### D) Dokumentieren
Neu/aktualisiert:
- `docs/LOCAL_FIRST_ROUTING_ARCHITECTURE.md`
- `docs/EXECUTION_PLAN_14D_LOCAL_FIRST.md`
- `README.md` (router dry-run + local-first value statement)

---

## No-Metric-No-Merge Block
- Unit tests: **22 passed / 0 failed**
- Router outcomes (expected baseline cases): **5/5 expected**
- Compile/lint quality gates: **green**
- Runtime overhead router dry-run: negligible (single local decision path)

## Release risk assessment
- Risk level: **low** (additive dry-run feature, no breaking change to existing condense flow)
- Rollback path: keep using existing `octk` condense mode (router mode optional)
