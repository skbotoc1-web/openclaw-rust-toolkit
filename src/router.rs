use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, ValueEnum, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum TaskClass {
    Embed,
    Classify,
    Extract,
    Summarize,
    Reasoning,
    HighRisk,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    LocalOnly,
    LocalThenCloud,
    CloudOnly,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RouterPolicy {
    pub policy_version: Option<String>,
    pub thresholds: Option<RouterThresholds>,
    pub defaults: Option<RouterDefaults>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RouterThresholds {
    pub min_local_confidence: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RouterDefaults {
    pub embed: Option<Route>,
    pub classify: Option<Route>,
    pub extract: Option<Route>,
    pub summarize: Option<Route>,
    pub reasoning: Option<Route>,
    pub high_risk: Option<Route>,
    pub unknown: Option<Route>,
}

#[derive(Clone, Debug)]
pub struct EffectiveRouterPolicy {
    pub policy_version: String,
    pub min_local_confidence: f64,
    pub defaults: EffectiveRouterDefaults,
}

#[derive(Clone, Debug)]
pub struct EffectiveRouterDefaults {
    pub embed: Route,
    pub classify: Route,
    pub extract: Route,
    pub summarize: Route,
    pub reasoning: Route,
    pub high_risk: Route,
    pub unknown: Route,
}

#[derive(Clone, Debug)]
pub struct RouteInput {
    pub task_class: TaskClass,
    pub confidence: Option<f64>,
    pub schema_valid: Option<bool>,
    pub budget_soft_limit_reached: bool,
    pub budget_hard_limit_reached: bool,
    pub cloud_required: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct RouteDecision {
    pub task_class: TaskClass,
    pub route: Route,
    pub reason_codes: Vec<String>,
    pub should_degrade_local: bool,
    pub policy_version: String,
}

impl Default for EffectiveRouterPolicy {
    fn default() -> Self {
        Self {
            policy_version: "router-v1".to_string(),
            min_local_confidence: 0.78,
            defaults: EffectiveRouterDefaults {
                embed: Route::LocalOnly,
                classify: Route::LocalThenCloud,
                extract: Route::LocalThenCloud,
                summarize: Route::LocalThenCloud,
                reasoning: Route::CloudOnly,
                high_risk: Route::CloudOnly,
                unknown: Route::LocalThenCloud,
            },
        }
    }
}

pub fn load_policy(path: Option<&Path>) -> Result<EffectiveRouterPolicy> {
    let Some(path) = path else {
        return Ok(EffectiveRouterPolicy::default());
    };

    let content = fs::read_to_string(path)
        .with_context(|| format!("failed reading router policy: {}", path.display()))?;
    let parsed: RouterPolicy = toml::from_str(&content)
        .with_context(|| format!("failed parsing router policy TOML: {}", path.display()))?;

    let mut effective = EffectiveRouterPolicy::default();

    if let Some(version) = parsed.policy_version {
        effective.policy_version = version;
    }

    if let Some(t) = parsed.thresholds
        && let Some(min_local_confidence) = t.min_local_confidence
    {
        effective.min_local_confidence = min_local_confidence;
    }

    if let Some(d) = parsed.defaults {
        if let Some(v) = d.embed {
            effective.defaults.embed = v;
        }
        if let Some(v) = d.classify {
            effective.defaults.classify = v;
        }
        if let Some(v) = d.extract {
            effective.defaults.extract = v;
        }
        if let Some(v) = d.summarize {
            effective.defaults.summarize = v;
        }
        if let Some(v) = d.reasoning {
            effective.defaults.reasoning = v;
        }
        if let Some(v) = d.high_risk {
            effective.defaults.high_risk = v;
        }
        if let Some(v) = d.unknown {
            effective.defaults.unknown = v;
        }
    }

    Ok(effective)
}

fn default_route_for(task: &TaskClass, policy: &EffectiveRouterPolicy) -> Route {
    match task {
        TaskClass::Embed => policy.defaults.embed.clone(),
        TaskClass::Classify => policy.defaults.classify.clone(),
        TaskClass::Extract => policy.defaults.extract.clone(),
        TaskClass::Summarize => policy.defaults.summarize.clone(),
        TaskClass::Reasoning => policy.defaults.reasoning.clone(),
        TaskClass::HighRisk => policy.defaults.high_risk.clone(),
        TaskClass::Unknown => policy.defaults.unknown.clone(),
    }
}

pub fn decide_route(input: &RouteInput, policy: &EffectiveRouterPolicy) -> RouteDecision {
    let mut reason_codes = Vec::<String>::new();
    let mut route = default_route_for(&input.task_class, policy);
    let mut should_degrade_local = false;

    if input.cloud_required {
        route = Route::CloudOnly;
        reason_codes.push("explicit_cloud_required".into());
    }

    if matches!(input.task_class, TaskClass::Embed) {
        route = Route::LocalOnly;
        reason_codes.push("embed_local_default".into());
    }

    if matches!(
        input.task_class,
        TaskClass::Classify | TaskClass::Extract | TaskClass::Summarize
    ) {
        if input.schema_valid == Some(false) {
            route = Route::LocalThenCloud;
            reason_codes.push("schema_validation_failed".into());
        } else if let Some(confidence) = input.confidence {
            if confidence >= policy.min_local_confidence {
                route = Route::LocalOnly;
                reason_codes.push("confidence_gate_passed".into());
            } else {
                route = Route::LocalThenCloud;
                reason_codes.push("confidence_gate_low".into());
            }
        } else {
            route = Route::LocalThenCloud;
            reason_codes.push("confidence_missing".into());
        }
    }

    if input.budget_soft_limit_reached && matches!(route, Route::CloudOnly) {
        route = Route::LocalThenCloud;
        reason_codes.push("budget_soft_limit_route_downshift".into());
    }

    if input.budget_hard_limit_reached && matches!(route, Route::CloudOnly | Route::LocalThenCloud)
    {
        route = Route::LocalOnly;
        should_degrade_local = true;
        reason_codes.push("budget_hard_limit_cloud_blocked".into());
    }

    if reason_codes.is_empty() {
        reason_codes.push("default_policy_route".into());
    }

    RouteDecision {
        task_class: input.task_class.clone(),
        route,
        reason_codes,
        should_degrade_local,
        policy_version: policy.policy_version.clone(),
    }
}

pub fn decision_log_json(decision: &RouteDecision) -> Result<String> {
    serde_json::to_string(decision).context("failed to serialize route decision")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_policy() -> EffectiveRouterPolicy {
        EffectiveRouterPolicy::default()
    }

    #[test]
    fn embed_always_routes_local_only() {
        let input = RouteInput {
            task_class: TaskClass::Embed,
            confidence: Some(0.2),
            schema_valid: Some(false),
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: true,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalOnly);
        assert!(
            decision
                .reason_codes
                .contains(&"embed_local_default".to_string())
        );
    }

    #[test]
    fn reasoning_routes_cloud_only_without_budget_pressure() {
        let input = RouteInput {
            task_class: TaskClass::Reasoning,
            confidence: None,
            schema_valid: None,
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::CloudOnly);
    }

    #[test]
    fn hard_budget_limit_blocks_cloud_and_degrades_local() {
        let input = RouteInput {
            task_class: TaskClass::Reasoning,
            confidence: None,
            schema_valid: None,
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: true,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalOnly);
        assert!(decision.should_degrade_local);
        assert!(
            decision
                .reason_codes
                .contains(&"budget_hard_limit_cloud_blocked".to_string())
        );
    }

    #[test]
    fn soft_budget_limit_downshifts_cloud_only() {
        let input = RouteInput {
            task_class: TaskClass::HighRisk,
            confidence: None,
            schema_valid: None,
            budget_soft_limit_reached: true,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalThenCloud);
        assert!(
            decision
                .reason_codes
                .contains(&"budget_soft_limit_route_downshift".to_string())
        );
    }

    #[test]
    fn classify_with_high_confidence_stays_local() {
        let input = RouteInput {
            task_class: TaskClass::Classify,
            confidence: Some(0.95),
            schema_valid: Some(true),
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalOnly);
        assert!(
            decision
                .reason_codes
                .contains(&"confidence_gate_passed".to_string())
        );
    }

    #[test]
    fn classify_with_low_confidence_uses_local_then_cloud() {
        let input = RouteInput {
            task_class: TaskClass::Classify,
            confidence: Some(0.35),
            schema_valid: Some(true),
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalThenCloud);
        assert!(
            decision
                .reason_codes
                .contains(&"confidence_gate_low".to_string())
        );
    }

    #[test]
    fn schema_failure_for_extract_forces_escalation_path() {
        let input = RouteInput {
            task_class: TaskClass::Extract,
            confidence: Some(0.99),
            schema_valid: Some(false),
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());

        assert_eq!(decision.route, Route::LocalThenCloud);
        assert!(
            decision
                .reason_codes
                .contains(&"schema_validation_failed".to_string())
        );
    }

    #[test]
    fn decisions_are_deterministic_for_identical_inputs() {
        let input = RouteInput {
            task_class: TaskClass::Summarize,
            confidence: Some(0.81),
            schema_valid: Some(true),
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };
        let policy = baseline_policy();

        let first = decide_route(&input, &policy);
        let second = decide_route(&input, &policy);

        assert_eq!(first.route, second.route);
        assert_eq!(first.reason_codes, second.reason_codes);
        assert_eq!(first.should_degrade_local, second.should_degrade_local);
    }

    #[test]
    fn route_decision_serializes_to_json_log() {
        let input = RouteInput {
            task_class: TaskClass::Unknown,
            confidence: None,
            schema_valid: None,
            budget_soft_limit_reached: false,
            budget_hard_limit_reached: false,
            cloud_required: false,
        };

        let decision = decide_route(&input, &baseline_policy());
        let log_line = decision_log_json(&decision).expect("serialize decision");

        assert!(log_line.contains("\"route\""));
        assert!(log_line.contains("\"policy_version\""));
    }
}
