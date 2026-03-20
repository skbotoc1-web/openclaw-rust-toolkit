---
name: "good first issue: artifact signing/provenance"
about: Add artifact signature and provenance attestation for releases
labels: ["good first issue", "security", "release"]
---

## Objective
Improve enterprise trust with signed releases and provenance.

## Tasks
- Add signing step for release artifacts
- Add provenance/attestation generation
- Document verification commands in docs

## Acceptance criteria
- Release artifacts are signed
- Verification instructions are documented and tested
- No secrets exposed in workflow logs
