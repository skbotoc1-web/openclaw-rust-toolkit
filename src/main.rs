use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "octk", version, about = "OpenClaw Rust Toolkit")]
struct Cli {
    #[arg(long, default_value = "auto")]
    mode: Mode,

    #[arg(long)]
    command: Option<String>,

    #[arg(long)]
    rules: Option<PathBuf>,

    #[arg(long, value_enum, default_value = "text")]
    report_format: ReportFormat,

    #[arg(long)]
    report_file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    emit_flag: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Auto,
    On,
    Off,
}

#[derive(Clone, Debug, ValueEnum)]
enum ReportFormat {
    Text,
    Json,
}

#[derive(Debug, Deserialize)]
struct RulesFile {
    activation: Option<ActivationRules>,
    condense: Option<CondenseRules>,
}

#[derive(Debug, Deserialize)]
struct ActivationRules {
    min_input_chars: Option<usize>,
    min_input_lines: Option<usize>,
    always_match: Option<Vec<String>>,
    never_match: Option<Vec<String>>,
    flag: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CondenseRules {
    head_lines: Option<usize>,
    tail_lines: Option<usize>,
    max_lines: Option<usize>,
    max_signal_lines: Option<usize>,
    dedupe: Option<bool>,
    signal_regex: Option<String>,
}

#[derive(Debug)]
struct EffectiveRules {
    min_input_chars: usize,
    min_input_lines: usize,
    always_match: Vec<String>,
    never_match: Vec<String>,
    flag: String,
    head_lines: usize,
    tail_lines: usize,
    max_lines: usize,
    max_signal_lines: usize,
    dedupe: bool,
    signal_regex: Regex,
}

#[derive(Debug, Serialize)]
struct UsageReport {
    used: bool,
    reason: String,
    command: Option<String>,
    input_chars: usize,
    output_chars: usize,
    input_lines: usize,
    output_lines: usize,
    saved_chars: usize,
    saved_percent: f64,
    estimated_input_tokens: usize,
    estimated_output_tokens: usize,
    flag: String,
}

fn default_rules() -> EffectiveRules {
    EffectiveRules {
        min_input_chars: 4000,
        min_input_lines: 120,
        always_match: vec![
            "openclaw logs".into(),
            "docker logs".into(),
            "git diff".into(),
            "find ".into(),
            "rg ".into(),
        ],
        never_match: vec!["cat secrets".into(), "private_key".into()],
        flag: "RUST_TOOLKIT_USED".into(),
        head_lines: 30,
        tail_lines: 30,
        max_lines: 180,
        max_signal_lines: 80,
        dedupe: true,
        signal_regex: Regex::new("(?i)error|warn|failed|exception|traceback|critical|denied|timeout|EACCES|ENOENT|unresolved")
            .expect("valid regex"),
    }
}

fn load_rules(path: Option<PathBuf>) -> Result<EffectiveRules> {
    let mut rules = default_rules();
    let Some(path) = path else {
        return Ok(rules);
    };

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed reading rules file: {}", path.display()))?;
    let parsed: RulesFile = toml::from_str(&content)
        .with_context(|| format!("failed parsing TOML rules: {}", path.display()))?;

    if let Some(a) = parsed.activation {
        if let Some(v) = a.min_input_chars {
            rules.min_input_chars = v;
        }
        if let Some(v) = a.min_input_lines {
            rules.min_input_lines = v;
        }
        if let Some(v) = a.always_match {
            rules.always_match = v;
        }
        if let Some(v) = a.never_match {
            rules.never_match = v;
        }
        if let Some(v) = a.flag {
            rules.flag = v;
        }
    }

    if let Some(c) = parsed.condense {
        if let Some(v) = c.head_lines {
            rules.head_lines = v;
        }
        if let Some(v) = c.tail_lines {
            rules.tail_lines = v;
        }
        if let Some(v) = c.max_lines {
            rules.max_lines = v;
        }
        if let Some(v) = c.max_signal_lines {
            rules.max_signal_lines = v;
        }
        if let Some(v) = c.dedupe {
            rules.dedupe = v;
        }
        if let Some(v) = c.signal_regex {
            rules.signal_regex = Regex::new(&v).context("invalid signal_regex")?;
        }
    }

    Ok(rules)
}

fn should_use_toolkit(
    mode: &Mode,
    command: Option<&str>,
    input: &str,
    r: &EffectiveRules,
) -> (bool, String) {
    match mode {
        Mode::On => return (true, "forced_on".into()),
        Mode::Off => return (false, "forced_off".into()),
        Mode::Auto => {}
    }

    let command = command.unwrap_or("").to_lowercase();
    if !command.is_empty() {
        if r.never_match
            .iter()
            .any(|n| command.contains(&n.to_lowercase()))
        {
            return (false, "never_match".into());
        }
        if r.always_match
            .iter()
            .any(|a| command.contains(&a.to_lowercase()))
        {
            return (true, "always_match".into());
        }
    }

    let chars = input.chars().count();
    let lines = input.lines().count();
    if chars >= r.min_input_chars || lines >= r.min_input_lines {
        return (true, "threshold".into());
    }

    (false, "below_threshold".into())
}

fn condense(input: &str, r: &EffectiveRules) -> String {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    if r.dedupe {
        let mut out: Vec<String> = Vec::with_capacity(lines.len());
        let mut prev: Option<&str> = None;
        for l in &lines {
            if prev == Some(l.as_str()) {
                continue;
            }
            out.push(l.clone());
            prev = Some(l);
        }
        lines = out;
    }

    if lines.is_empty() {
        return String::new();
    }

    let head: Vec<String> = lines.iter().take(r.head_lines).cloned().collect();
    let tail: Vec<String> = lines
        .iter()
        .rev()
        .take(r.tail_lines)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let head_set: HashSet<&str> = head.iter().map(String::as_str).collect();
    let tail_set: HashSet<&str> = tail.iter().map(String::as_str).collect();

    let mut signal: Vec<String> = lines
        .iter()
        .filter(|l| r.signal_regex.is_match(l))
        .filter(|l| !head_set.contains(l.as_str()) && !tail_set.contains(l.as_str()))
        .take(r.max_signal_lines)
        .cloned()
        .collect();

    let mut result = Vec::<String>::new();
    result.extend(head);
    if !signal.is_empty() {
        result.push("… [signal lines] …".into());
        result.append(&mut signal);
    }
    if !tail.is_empty() {
        result.push("… [tail] …".into());
        result.extend(tail);
    }

    if result.len() > r.max_lines {
        let keep = r.max_lines / 2;
        let mut clipped = Vec::new();
        clipped.extend(result.iter().take(keep).cloned());
        clipped.push("… [truncated] …".into());
        clipped.extend(
            result
                .iter()
                .rev()
                .take(keep)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev(),
        );
        result = clipped;
    }

    result.join("\n")
}

fn build_report(
    used: bool,
    reason: String,
    command: Option<String>,
    input: &str,
    output: &str,
    flag: String,
) -> UsageReport {
    let input_chars = input.chars().count();
    let output_chars = output.chars().count();
    let input_lines = if input.is_empty() {
        0
    } else {
        input.lines().count()
    };
    let output_lines = if output.is_empty() {
        0
    } else {
        output.lines().count()
    };
    let saved_chars = input_chars.saturating_sub(output_chars);
    let saved_percent = if input_chars == 0 {
        0.0
    } else {
        (saved_chars as f64 / input_chars as f64) * 100.0
    };

    UsageReport {
        used,
        reason,
        command,
        input_chars,
        output_chars,
        input_lines,
        output_lines,
        saved_chars,
        saved_percent,
        estimated_input_tokens: input_chars / 4,
        estimated_output_tokens: output_chars / 4,
        flag,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let rules = load_rules(cli.rules)?;

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("failed to read stdin")?;

    let (should_use, reason) =
        should_use_toolkit(&cli.mode, cli.command.as_deref(), &input, &rules);
    let mut used = should_use;
    let mut final_reason = reason;

    let output = if should_use {
        let condensed = condense(&input, &rules);
        if condensed.chars().count() >= input.chars().count() {
            used = false;
            final_reason = "no_gain".into();
            input.clone()
        } else {
            condensed
        }
    } else {
        input.clone()
    };

    print!("{}", output);

    let report = build_report(
        used,
        final_reason,
        cli.command.clone(),
        &input,
        &output,
        rules.flag.clone(),
    );

    let rendered = match cli.report_format {
        ReportFormat::Json => serde_json::to_string_pretty(&report)?,
        ReportFormat::Text => format!(
            "[{}] used={} reason={} saved={:.1}% chars:{}->{} tokens~:{}->{}",
            report.flag,
            report.used,
            report.reason,
            report.saved_percent,
            report.input_chars,
            report.output_chars,
            report.estimated_input_tokens,
            report.estimated_output_tokens
        ),
    };

    if cli.emit_flag {
        eprintln!("{}", rendered);
    }

    if let Some(path) = cli.report_file {
        let json = serde_json::to_string_pretty(&report)?;
        fs::write(&path, json)
            .with_context(|| format!("failed to write report file: {}", path.display()))?;
    }

    Ok(())
}
