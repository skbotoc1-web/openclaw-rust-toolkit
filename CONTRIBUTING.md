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
- [ ] Explain why this improves token reduction and/or safe adoption
- [ ] Include before/after metrics when behavior changes
- [ ] Include runtime overhead impact (`time` before/after)
- [ ] Keep default path simple (avoid feature creep)
- [ ] Keep default rules conservative for debugging safety
- [ ] Confirm alignment with `docs/PRODUCT_GUARDRAILS.md`

### Mandatory PR metric block (No metric, no merge)
```text
Input tokens~: <x>
Output tokens~: <y>
Reduction: <z>%
Runtime overhead: <t>
```
