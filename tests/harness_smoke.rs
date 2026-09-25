//! Smoke test for `agentic-ledger`.
//!
//! Covers the 4-part acceptance spec in
//! `octos-arc-ref/docs/OCTOS_HARNESS_DEVELOPER_GUIDE.md` Part 2 Step 5, plus
//! the failure paths the hackathon requires us to be able to show.

use std::path::{Path, PathBuf};

use agentic_ledger::{AnalyzeInput, IngestInput, analyze_spending, categorize, ingest_expenses};
use octos_agent::task_supervisor::{TaskLifecycleState, TaskRuntimeState, TaskSupervisor};
use octos_agent::workspace_policy::{WorkspacePolicy, WorkspacePolicyKind};
use octos_plugin::PluginManifest;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

const SAMPLE: &str = "2026-09-01 美团外卖 45.90\n2026-09-02 地铁 6.00\n2026-09-03 中石化加油 300.00\n";

// ---- Part 1: manifest ----------------------------------------------------

#[test]
fn manifest_parses_and_declares_both_tools() {
    let manifest = PluginManifest::from_file(&crate_root().join("manifest.json"))
        .expect("manifest.json must parse");
    assert_eq!(manifest.id, "agentic-ledger");

    for name in ["ingest_expenses", "analyze_spending"] {
        let tool = manifest
            .tools
            .iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| panic!("{name} tool missing"));
        assert!(
            tool.input_schema
                .get("properties")
                .and_then(|v| v.as_object())
                .is_some_and(|m| !m.is_empty()),
            "{name} must declare non-empty input_schema.properties"
        );
    }
}

// ---- Part 2: workspace policy -------------------------------------------

#[test]
fn workspace_policy_parses_and_binds_analyze_to_primary() {
    let raw = std::fs::read_to_string(crate_root().join("workspace-policy.toml"))
        .expect("policy readable");
    let policy: WorkspacePolicy = toml::from_str(&raw).expect("policy parses");

    assert_eq!(policy.workspace.kind, WorkspacePolicyKind::Session);
    assert_eq!(
        policy.artifacts.entries.get("primary").map(String::as_str),
        Some("reports/spending-*.md")
    );

    let task = policy
        .spawn_tasks
        .get("analyze_spending")
        .expect("spawn_tasks.analyze_spending");
    assert_eq!(task.artifact_sources(), vec!["primary"]);
    assert!(
        task.on_verify
            .iter()
            .any(|action| action == "file_size_min:$primary:256"),
        "must declare file_size_min validator"
    );
    assert!(
        task.on_failure.iter().any(|a| a.starts_with("notify_user:")),
        "on_failure must notify the operator"
    );
}

// ---- Part 3: artifact production ----------------------------------------

#[test]
fn full_pipeline_produces_a_checkable_primary_artifact() {
    let tmp = tempfile::tempdir().unwrap();

    let ingested = ingest_expenses(
        tmp.path(),
        &IngestInput { text: SAMPLE.into() },
    )
    .expect("ingest must succeed on valid input");
    assert_eq!(ingested.record_count, 3);

    let out = analyze_spending(
        tmp.path(),
        &AnalyzeInput {
            label: "September W1".into(),
        },
    )
    .expect("analyze must succeed");

    let body = std::fs::read_to_string(tmp.path().join(&out.artifact_path)).unwrap();
    assert!(body.starts_with("# 消费分析报告：September W1"));
    // ¥45.90 + ¥6.00 + ¥300.00 — integer cents, exactly reproducible.
    assert_eq!(out.total_cents, 35_190);
    assert!(body.contains("¥351.90"), "total must appear in the report");
    assert!(body.len() >= 256, "must satisfy file_size_min:256");

    let raw = std::fs::read_to_string(crate_root().join("workspace-policy.toml")).unwrap();
    let policy: WorkspacePolicy = toml::from_str(&raw).unwrap();
    let pattern = policy.artifacts.entries.get("primary").unwrap();
    assert!(
        path_matches_glob(&out.artifact_path, pattern),
        "produced path {} must match primary glob {pattern}",
        out.artifact_path.display()
    );
}

#[test]
fn categories_are_rule_based_and_never_silent() {
    assert_eq!(categorize("美团外卖"), "餐饮");
    assert_eq!(categorize("地铁出行"), "交通");
    assert_eq!(categorize("未知商户xyz"), "其他");
}

// ---- Failure / empty states (hackathon requires these be demoable) -------

#[test]
fn ingest_rejects_input_with_no_parseable_lines() {
    let tmp = tempfile::tempdir().unwrap();
    let err = ingest_expenses(
        tmp.path(),
        &IngestInput {
            text: "# comment only\n\nnot a transaction\n".into(),
        },
    )
    .expect_err("empty input must fail");
    assert!(
        err.to_string().contains("no transactions parsed"),
        "unexpected error: {err:?}"
    );
    assert!(
        !tmp.path().join("data/transactions.json").exists(),
        "a failed ingest must not leave a dataset behind"
    );
}

#[test]
fn analyze_refuses_to_run_without_a_dataset() {
    let tmp = tempfile::tempdir().unwrap();
    let err = analyze_spending(
        tmp.path(),
        &AnalyzeInput { label: "orphan".into() },
    )
    .expect_err("analyze before ingest must fail");
    assert!(
        err.to_string().contains("run ingest_expenses first"),
        "unexpected error: {err:?}"
    );
}

// ---- Part 4: lifecycle ---------------------------------------------------

#[test]
fn lifecycle_state_transitions_queued_running_verifying_ready() {
    let supervisor = TaskSupervisor::new();
    let id = supervisor.register("analyze_spending", "call-ledger", None);

    assert_eq!(
        supervisor.get_task(&id).unwrap().lifecycle_state(),
        TaskLifecycleState::Queued,
    );
    supervisor.mark_running(&id);
    assert_eq!(
        supervisor.get_task(&id).unwrap().lifecycle_state(),
        TaskLifecycleState::Running,
    );
    supervisor.mark_runtime_state(
        &id,
        TaskRuntimeState::VerifyingOutputs,
        Some("verify analyze_spending primary".to_string()),
    );
    assert_eq!(
        supervisor.get_task(&id).unwrap().lifecycle_state(),
        TaskLifecycleState::Verifying,
    );
    supervisor.mark_completed(&id, vec!["reports/spending-september-w1.md".to_string()]);
    assert_eq!(
        supervisor.get_task(&id).unwrap().lifecycle_state(),
        TaskLifecycleState::Ready,
    );
}

#[test]
fn lifecycle_reaches_failed_when_verification_fails() {
    let supervisor = TaskSupervisor::new();
    let id = supervisor.register("analyze_spending", "call-empty", None);
    supervisor.mark_running(&id);
    supervisor.mark_failed(&id, "file_exists:$primary check failed".to_string());

    let task = supervisor.get_task(&id).unwrap();
    assert_eq!(task.lifecycle_state(), TaskLifecycleState::Failed);
}

fn path_matches_glob(path: &Path, pattern: &str) -> bool {
    // `\` -> `/` so the `/`-based policy globs match on Windows, where
    // to_string_lossy() renders backslash separators.
    let path_str = path.to_string_lossy().replace('\\', "/");
    let (prefix, rest) = match pattern.split_once('*') {
        Some(pair) => pair,
        None => return path_str == pattern,
    };
    path_str.starts_with(prefix) && path_str[prefix.len()..].ends_with(rest)
}
