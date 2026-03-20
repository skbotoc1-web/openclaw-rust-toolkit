# Local Compute & Memory Requirements

## Runtime (using prebuilt `octk` binary)

### Minimum
- CPU: 1 vCPU
- RAM: 256 MB free
- Disk: ~20 MB (binary + small report files)
- OS: Linux/macOS/Windows (x64)

### Recommended
- CPU: 2 vCPU+
- RAM: 512 MB free
- Disk: 100 MB free (logs + reports + tooling comfort)

## Build from source (cargo)

### Minimum
- CPU: 2 vCPU
- RAM: 2 GB
- Disk: 1.5 GB free (Rust toolchain + build cache)

### Recommended
- CPU: 4 vCPU+
- RAM: 4 GB
- Disk: 3 GB free

## Performance expectations
- Typical overhead for condensing is low vs. large command output size.
- Biggest gains appear on large logs/search outputs.
- For small outputs, toolkit may auto-skip (`reason=no_gain`) to avoid unnecessary processing.

## What users should expect
- More stable token/cost usage for noisy CLI output
- Better context headroom for LLM calls
- No hard dependency on GPU

## Notes
- GPU is not required.
- If running in CI or low-memory containers, prefer prebuilt release binary over source build.
