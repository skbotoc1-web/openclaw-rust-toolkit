---
name: "good first issue: linux arm64 release binary"
about: Add prebuilt Linux ARM64 artifact to release flow
labels: ["good first issue", "release", "arm64"]
---

## Objective
Ship Linux ARM64 prebuilt binaries in release assets.

## Tasks
- Add reliable ARM64 build strategy to release workflow
- Ensure checksum artifact is generated
- Verify download/install path in docs

## Acceptance criteria
- Release contains `octk-aarch64-unknown-linux-gnu` artifact
- Matching checksum file exists
- CI/release pipeline remains green
