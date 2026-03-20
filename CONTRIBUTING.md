# Contributing

Thanks for helping improve OpenClaw Rust Toolkit.

## What we welcome
- Better activation heuristics
- Safer signal-preservation logic
- Benchmarks from real-world OpenClaw workloads
- Integration examples for different environments

## Dev setup
```bash
cargo build
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

## Pull request checklist
- [ ] Explain why this improves signal/cost tradeoff
- [ ] Include before/after metrics when behavior changes
- [ ] Keep default rules conservative for debugging safety
