---
name: "good first issue: behavioral tests"
about: Add behavioral tests to harden core decision logic
labels: ["good first issue", "tests"]
---

## Objective
Increase confidence in token-reduction behavior and regression safety.

## Tasks
- Add unit tests for profile behavior differences
- Add tests for marker/report consistency
- Add edge-case tests for no-gain fallback and signal retention

## Acceptance criteria
- `cargo test --all` passes
- New tests cover meaningful behavior paths
- No existing behavior regressions
