# Implementation Spec

## Binary
`octk`

## Input/Output
- Input: STDIN (raw command output)
- Output: STDOUT (condensed or passthrough)
- Report: STDERR marker and optional JSON file

## CLI
- `--mode auto|on|off`
- `--command <string>`
- `--rules <path.toml>`
- `--report-format text|json`
- `--report-file <path>`
- `--emit-flag true|false`

## Algorithm (Condense)
1. Optional dedupe of neighboring duplicate lines
2. Keep head/tail windows
3. Keep signal lines from middle
4. Clip to global max lines
5. Preserve deterministic order

## Non-Goals
- Semantic summarization by LLM
- Mutating command content
- Parsing provider-specific logs

## QA checklist
- [ ] mode=off gives byte-identical passthrough
- [ ] mode=on emits marker and report
- [ ] auto triggers on always_match
- [ ] auto skips on never_match
- [ ] thresholds work as documented
- [ ] signal lines survive clipping
- [ ] report file is valid JSON
