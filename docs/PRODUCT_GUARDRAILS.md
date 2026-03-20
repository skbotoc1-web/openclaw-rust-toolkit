# Product Guardrails

## Core mission
Reduce token consumption from noisy CLI output **without breaking debugging quality**.

## Success metrics (must track)
1. Token reduction (%)
2. No-gain rate (`reason=no_gain`)
3. Missed-signal incidents (debug quality regressions)
4. Runtime overhead

## Merge policy
A change should be merged only if at least one is true:
- Improves token reduction for target workloads, or
- Improves reliability/upgrade-safety/security with no material regression, or
- Improves enterprise readiness gates (tests, integrity, provenance)

And all of the following must hold:
- No critical debug-signal regression
- No secrets exposure risk
- Build/lint/tests pass

## Hard rule: No metric, no merge
Every behavior-impacting PR must include a before/after metric block:
- input tokens (or chars/4 estimate)
- output tokens
- reduction %
- runtime overhead

If this block is missing, PR is blocked by policy.

## Anti-feature-creep rules
- No feature that does not map to core mission or enterprise gates
- Prefer simplification over optional complexity
- Keep default path minimal: install -> auto/balanced -> measurable savings

## Release gate (practical)
Before release:
1. Run benchmark sample and report before/after tokens
2. Verify `RUST_TOOLKIT_DETECTED` + `RUST_TOOLKIT_USED` markers
3. Verify deploy-health policy check
4. Verify checksum artifacts are published

## Decision rule for new ideas
If an idea cannot answer "how does this reduce tokens or strengthen safe adoption?" in one sentence, it should not be prioritized.
