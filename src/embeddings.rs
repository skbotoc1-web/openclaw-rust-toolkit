use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::fs::{self, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub model: String,
    pub dims: usize,
    pub text_chars: usize,
    pub vector: Vec<f32>,
    pub route: String,
    pub cloud_calls: u32,
    pub reason_codes: Vec<String>,
    pub created_at_unix: u64,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingHealth {
    pub local_available: bool,
    pub model: String,
    pub cloud_calls: u32,
    pub reason_codes: Vec<String>,
}

pub fn local_embedding_health(model: &str) -> EmbeddingHealth {
    EmbeddingHealth {
        local_available: true,
        model: model.to_string(),
        cloud_calls: 0,
        reason_codes: vec!["local_embedding_lane_ready".to_string()],
    }
}

pub fn build_local_embedding_record(
    text: &str,
    dim: usize,
    id: Option<&str>,
    model: &str,
) -> Result<EmbeddingRecord> {
    let clean_dim = dim.max(8);
    let vector = embed_text_hash(text, clean_dim);
    let generated_id = id
        .map(ToString::to_string)
        .unwrap_or_else(|| stable_text_id(text, clean_dim));

    let created_at_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before unix epoch")?
        .as_secs();

    Ok(EmbeddingRecord {
        id: generated_id,
        model: model.to_string(),
        dims: clean_dim,
        text_chars: text.chars().count(),
        vector,
        route: "local_only".to_string(),
        cloud_calls: 0,
        reason_codes: vec!["local_embedding_lane".to_string()],
        created_at_unix,
    })
}

pub fn append_record_jsonl(path: &Path, record: &EmbeddingRecord) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create index directory: {}", parent.display()))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open index file: {}", path.display()))?;

    let line = serde_json::to_string(record).context("failed to serialize embedding record")?;
    writeln!(file, "{}", line)
        .with_context(|| format!("failed to append index file: {}", path.display()))?;

    Ok(())
}

fn stable_text_id(text: &str, dim: usize) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    dim.hash(&mut hasher);
    format!("emb-{:016x}", hasher.finish())
}

fn embed_text_hash(text: &str, dim: usize) -> Vec<f32> {
    let mut bins = vec![0.0_f32; dim];
    if text.trim().is_empty() {
        return bins;
    }

    for token in text.split_whitespace() {
        let lower = token.to_ascii_lowercase();
        let idx = stable_index(&lower, dim);
        bins[idx] += 1.0;

        if lower.len() >= 4 {
            let prefix = &lower[..4];
            let idx2 = stable_index(prefix, dim);
            bins[idx2] += 0.5;
        }
    }

    l2_normalize(&mut bins);
    bins
}

fn stable_index(token: &str, dim: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    (hasher.finish() as usize) % dim
}

fn l2_normalize(v: &mut [f32]) {
    let norm_sq: f32 = v.iter().map(|x| x * x).sum();
    if norm_sq <= f32::EPSILON {
        return;
    }
    let norm = norm_sq.sqrt();
    for x in v.iter_mut() {
        *x /= norm;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedding_has_requested_dim() {
        let rec = build_local_embedding_record("hello world", 64, None, "octk-local-hash-v1")
            .expect("record");
        assert_eq!(rec.vector.len(), 64);
        assert_eq!(rec.dims, 64);
    }

    #[test]
    fn embedding_is_deterministic_for_same_input() {
        let r1 = build_local_embedding_record("same text", 32, None, "octk-local-hash-v1")
            .expect("record1");
        let r2 = build_local_embedding_record("same text", 32, None, "octk-local-hash-v1")
            .expect("record2");

        assert_eq!(r1.id, r2.id);
        assert_eq!(r1.vector, r2.vector);
    }

    #[test]
    fn embedding_route_is_local_and_cloud_calls_zero() {
        let rec = build_local_embedding_record("abc", 16, Some("x"), "octk-local-hash-v1")
            .expect("record");
        assert_eq!(rec.route, "local_only");
        assert_eq!(rec.cloud_calls, 0);
        assert!(
            rec.reason_codes
                .contains(&"local_embedding_lane".to_string())
        );
    }

    #[test]
    fn health_check_reports_local_ready() {
        let health = local_embedding_health("octk-local-hash-v1");
        assert!(health.local_available);
        assert_eq!(health.cloud_calls, 0);
    }

    #[test]
    fn append_record_writes_jsonl() {
        let temp_dir = std::env::temp_dir().join("octk-embeddings-test");
        let path = temp_dir.join("index.jsonl");
        let _ = fs::remove_file(&path);

        let rec = build_local_embedding_record("hello", 16, Some("id-1"), "octk-local-hash-v1")
            .expect("record");
        append_record_jsonl(&path, &rec).expect("append");

        let content = fs::read_to_string(&path).expect("read back");
        assert!(content.contains("\"id\":\"id-1\""));
    }
}
