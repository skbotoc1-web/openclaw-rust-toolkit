# PR2 — Local Embeddings Lane

## Goal
Run embedding requests locally by default and write vectors into a local index path without cloud API calls.

## Delivered
- Local embedding module: `src/embeddings.rs`
- Deterministic hash-based local embedding generator
- Health check mode for local embedding lane
- JSONL index append path for local RAG ingestion
- CLI integration in `octk`

## CLI
### Health check
```bash
octk --embed-health-check --emit-embed-log
```

### Inline input embedding
```bash
octk \
  --embed-input "OpenClaw local embedding test" \
  --embed-dim 64 \
  --embed-model octk-local-hash-v1 \
  --embed-index-path ./rag/local-index.jsonl \
  --emit-embed-log
```

### File input embedding
```bash
octk \
  --embed-input-file ./docs/sample.txt \
  --embed-index-path ./rag/local-index.jsonl
```

## Output contract
The embedding output is JSON with:
- `route: "local_only"`
- `cloud_calls: 0`
- `reason_codes` containing `local_embedding_lane`
- deterministic `id`, `dims`, and `vector`

## Notes
- This lane is intentionally local-only for embeddings.
- Cloud fallback for embeddings is not used in PR2 by design.
- PR3/PR4 will add broader local inference + escalation control paths for non-embedding tasks.
