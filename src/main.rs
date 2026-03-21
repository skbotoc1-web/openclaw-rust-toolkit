use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
}

fn parse_bool_flag(v: &str) -> bool {
    matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "on" | "yes")
}

fn effective_log_level() -> LogLevel {
    let debug_on = env::var("OCTK_DEBUG")
        .ok()
        .map(|v| parse_bool_flag(&v))
        .unwrap_or_else(|| {
            env::var("OPENCLAW_DEBUG")
                .ok()
                .map(|v| parse_bool_flag(&v))
                .unwrap_or(false)
        });

    if debug_on {
        return LogLevel::Debug;
    }

    let raw = env::var("OCTK_LOG_LEVEL")
        .ok()
        .or_else(|| env::var("OPENCLAW_LOG_LEVEL").ok())
        .unwrap_or_else(|| "warn".to_string())
        .to_ascii_lowercase();

    match raw.as_str() {
        "error" => LogLevel::Error,
        "warn" | "warning" => LogLevel::Warn,
        "info" => LogLevel::Info,
        "debug" => LogLevel::Debug,
        _ => LogLevel::Warn,
    }
}

fn log_msg(level: LogLevel, msg: &str) {
    if level <= effective_log_level() {
        let label = match level {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        };
        eprintln!("[octk][{}] {}", label, msg);
    }
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

fn process_input(
    mode: &Mode,
    command: Option<&str>,
    input: &str,
    rules: &EffectiveRules,
) -> (String, bool, String) {
    let (should_use, reason) = should_use_toolkit(mode, command, input, rules);
    let mut used = should_use;
    let mut final_reason = reason;

    let output = if should_use {
        let condensed = condense(input, rules);
        if condensed.chars().count() >= input.chars().count() {
            used = false;
            final_reason = "no_gain".into();
            input.to_string()
        } else {
            condensed
        }
    } else {
        input.to_string()
    };

    (output, used, final_reason)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let rules = load_rules(cli.rules)?;

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("failed to read stdin")?;

    log_msg(
        LogLevel::Debug,
        &format!(
            "input_received chars={} lines={} mode={:?} command={}",
            input.chars().count(),
            input.lines().count(),
            cli.mode,
            cli.command.as_deref().unwrap_or("<none>")
        ),
    );

    let (output, used, final_reason) =
        process_input(&cli.mode, cli.command.as_deref(), &input, &rules);

    log_msg(
        LogLevel::Info,
        &format!(
            "decision used={} reason={} chars:{}->{} tokens~:{}->{}",
            used,
            final_reason,
            input.chars().count(),
            output.chars().count(),
            input.chars().count() / 4,
            output.chars().count() / 4
        ),
    );

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
        log_msg(
            LogLevel::Debug,
            &format!("report_written path={}", path.display()),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_on_forces_use() {
        let rules = default_rules();
        let (use_toolkit, reason) = should_use_toolkit(&Mode::On, None, "abc", &rules);
        assert!(use_toolkit);
        assert_eq!(reason, "forced_on");
    }

    #[test]
    fn mode_off_forces_skip() {
        let rules = default_rules();
        let (use_toolkit, reason) = should_use_toolkit(&Mode::Off, None, "abc", &rules);
        assert!(!use_toolkit);
        assert_eq!(reason, "forced_off");
    }

    #[test]
    fn never_match_overrides_always_match() {
        let rules = default_rules();
        let command = "openclaw logs && cat secrets";
        let (use_toolkit, reason) = should_use_toolkit(&Mode::Auto, Some(command), "abc", &rules);
        assert!(!use_toolkit);
        assert_eq!(reason, "never_match");
    }

    #[test]
    fn threshold_triggers_when_input_is_large() {
        let mut rules = default_rules();
        rules.min_input_chars = 10;
        let input = "12345678901";
        let (use_toolkit, reason) = should_use_toolkit(&Mode::Auto, None, input, &rules);
        assert!(use_toolkit);
        assert_eq!(reason, "threshold");
    }

    #[test]
    fn condense_keeps_signal_lines() {
        let mut rules = default_rules();
        rules.head_lines = 1;
        rules.tail_lines = 1;
        let input = "head\nnoise\nERROR boom\nmore\ntail";
        let out = condense(input, &rules);
        assert!(out.contains("ERROR boom"));
    }

    #[test]
    fn no_gain_falls_back_to_original() {
        let rules = default_rules();
        let input = "only-one-line";
        let (output, used, reason) = process_input(&Mode::On, Some("openclaw logs"), input, &rules);
        assert_eq!(output, input);
        assert!(!used);
        assert_eq!(reason, "no_gain");
    }

    #[test]
    fn report_fields_are_consistent() {
        let report = build_report(
            true,
            "always_match".into(),
            Some("openclaw logs".into()),
            "abcd",
            "ab",
            "RUST_TOOLKIT_USED".into(),
        );
        assert_eq!(report.saved_chars, 2);
        assert_eq!(report.estimated_input_tokens, 1);
        assert_eq!(report.estimated_output_tokens, 0);
        assert_eq!(report.flag, "RUST_TOOLKIT_USED");
        assert!(report.saved_percent > 0.0);
    }

    #[test]
    fn always_match_beats_thresholds_in_auto_mode() {
        let mut rules = default_rules();
        rules.min_input_chars = usize::MAX;
        rules.min_input_lines = usize::MAX;

        let (use_toolkit, reason) = should_use_toolkit(
            &Mode::Auto,
            Some("docker logs my-service"),
            "tiny",
            &rules,
        );

        assert!(use_toolkit);
        assert_eq!(reason, "always_match");
    }

    #[test]
    fn threshold_uses_line_count_when_char_count_is_low() {
        let mut rules = default_rules();
        rules.min_input_chars = usize::MAX;
        rules.min_input_lines = 3;

        let input = "a\nb\nc";
        let (use_toolkit, reason) = should_use_toolkit(&Mode::Auto, None, input, &rules);

        assert!(use_toolkit);
        assert_eq!(reason, "threshold");
    }

    #[test]
    fn condense_dedupes_consecutive_lines_when_enabled() {
        let mut rules = default_rules();
        rules.head_lines = 10;
        rules.tail_lines = 0;
        rules.max_lines = 200;
        rules.max_signal_lines = 0;
        rules.dedupe = true;

        let input = "same\nsame\nother\nother\nend";
        let output = condense(input, &rules);

        assert!(!output.contains("same\nsame"));
        assert!(!output.contains("other\nother"));
        assert!(output.contains("same\nother\nend"));
    }

    #[test]
    fn condense_without_signals_omits_signal_marker() {
        let mut rules = default_rules();
        rules.head_lines = 1;
        rules.tail_lines = 1;

        let input = "head\nneutral line\ntail";
        let output = condense(input, &rules);

        assert!(!output.contains("… [signal lines] …"));
        assert!(output.contains("… [tail] …"));
    }

    #[test]
    fn process_input_keeps_full_text_and_reason_when_not_used() {
        let mut rules = default_rules();
        rules.min_input_chars = usize::MAX;
        rules.min_input_lines = usize::MAX;

        let input = "short input";
        let (output, used, reason) = process_input(&Mode::Auto, None, input, &rules);

        assert_eq!(output, input);
        assert!(!used);
        assert_eq!(reason, "below_threshold");
    }

    #[test]
    fn report_is_zeroed_for_empty_input() {
        let report = build_report(false, "below_threshold".into(), None, "", "", "FLAG".into());

        assert_eq!(report.input_chars, 0);
        assert_eq!(report.output_chars, 0);
        assert_eq!(report.input_lines, 0);
        assert_eq!(report.output_lines, 0);
        assert_eq!(report.saved_chars, 0);
        assert_eq!(report.saved_percent, 0.0);
    }
}
