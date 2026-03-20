# Compatibility Matrix

| Component | Version / Target | Status |
|---|---|---|
| octk | v0.1.x | ✅ |
| OpenClaw CLI | 2026.3.x (tested with 2026.3.13) | ✅ |
| Linux | x86_64-unknown-linux-gnu | ✅ |
| Linux | aarch64-unknown-linux-gnu | ⚠️ source-build fallback |
| macOS | x86_64-apple-darwin | ✅ (release target) |
| macOS | aarch64-apple-darwin | ✅ (release target) |
| Windows | x86_64-pc-windows-msvc | ✅ (release target) |

## Notes
- Release binaries are published on each tag `v*` via GitHub Actions.
- If target-specific runtime issues appear, mark status with ⚠️ and document workaround.
