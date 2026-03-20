# Full Audit Loop 2 — 2026-03-20

Repo: `skbotoc1-web/openclaw-rust-toolkit`

## Scope
- Code quality + build health
- Secret hygiene
- Deploy-health automation
- Profile behavior validation
- Workflow runtime deprecation risk

## Executed checks
- `cargo fmt --all --check` ✅
- `cargo clippy --all-targets --all-features -- -D warnings` ✅
- `cargo test --all` ✅ (still 0 tests)
- `scripts/secret-check.sh` ✅ (no secrets leaked)
- `scripts/last-deploy-status.sh --require-success --min-assets 3 --max-age-hours 720` ✅
- Manual run of `Deploy Health` workflow ✅

## Findings

### F1 — Workflow runtime warning (fixed)
- Observation: `actions/checkout@v4` emitted Node20 deprecation warning in Deploy Health run.
- Fix: migrated all workflows to `actions/checkout@v5`.
- Files updated:
  - `.github/workflows/ci.yml`
  - `.github/workflows/release.yml`
  - `.github/workflows/deploy-health.yml`

### F2 — Profile behavior check (validated)
Input: `openclaw logs --limit 400 --plain`

| Profile | used | reason | estimated tokens (in -> out) | savings |
|---|---|---|---:|---:|
| safe | true | always_match | 12923 -> 3418 | 73.5% |
| balanced | true | always_match | 12923 -> 2518 | 80.5% |
| aggressive | true | always_match | 12923 -> 1895 | 85.3% |

Result: profile ladder works as intended (`safe < balanced < aggressive` compression).

## Open items (unchanged)
- Add behavioral unit tests (currently 0 tests)
- Linux ARM64 prebuilt release asset (currently source fallback)
- Optional: release `SHA256SUMS` manifest

## Conclusion
Second audit loop is successful. No new critical/high risks. One medium operational warning was fixed (checkout action upgrade).
