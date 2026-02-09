//! CLI determinism tests.
//!
//! Validates that the `context build` + `context resolve` pipeline
//! produces byte-identical JSON output across multiple runs,
//! matching the central determinism invariant.

use std::fs;
use std::process::Command;

fn context_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_context"))
}

/// Create a set of fixture .md files in `dir`.
fn write_fixture_sources(dir: &std::path::Path) {
    let files = [
        (
            "docs/api.md",
            "API reference for the context platform REST endpoints and authentication",
        ),
        (
            "docs/deployment.md",
            "Deployment guide for production environments including Docker and Kubernetes",
        ),
        (
            "docs/architecture.md",
            "System architecture overview describing the cache compiler pipeline",
        ),
        (
            "docs/quickstart.md",
            "Getting started with context resolve in five minutes",
        ),
    ];

    for (rel_path, content) in &files {
        let path = dir.join(rel_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
    }
}

#[test]
fn build_then_resolve_is_deterministic() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    write_fixture_sources(&sources);

    let cache = tmp.path().join("cache");

    // Build the cache
    let build_output = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .expect("failed to run context build");

    assert!(
        build_output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Resolve twice with the same query and budget
    let resolve_a = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache)
        .args(["--query", "deployment", "--budget", "4096"])
        .output()
        .expect("failed to run context resolve (a)");

    let resolve_b = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache)
        .args(["--query", "deployment", "--budget", "4096"])
        .output()
        .expect("failed to run context resolve (b)");

    assert!(resolve_a.status.success(), "resolve (a) failed");
    assert!(resolve_b.status.success(), "resolve (b) failed");

    assert_eq!(
        resolve_a.stdout, resolve_b.stdout,
        "Two resolve runs must produce byte-identical stdout"
    );

    // Verify it's valid JSON with the expected structure
    let output: serde_json::Value =
        serde_json::from_slice(&resolve_a.stdout).expect("resolve output is not valid JSON");

    assert!(output.get("documents").is_some(), "Missing 'documents' key");
    assert!(output.get("selection").is_some(), "Missing 'selection' key");

    let selection = output.get("selection").unwrap();
    assert_eq!(selection["query"].as_str().unwrap(), "deployment");
    assert_eq!(selection["budget"].as_u64().unwrap(), 4096);
}

#[test]
fn rebuild_produces_identical_resolve_output() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    write_fixture_sources(&sources);

    // Build #1
    let cache_1 = tmp.path().join("cache1");
    let out_1 = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache_1)
        .output()
        .unwrap();
    assert!(out_1.status.success());

    // Build #2 (same sources, fresh cache dir)
    let cache_2 = tmp.path().join("cache2");
    let out_2 = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache_2)
        .output()
        .unwrap();
    assert!(out_2.status.success());

    // Resolve from each cache
    let resolve_1 = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache_1)
        .args(["--query", "architecture", "--budget", "4096"])
        .output()
        .unwrap();

    let resolve_2 = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache_2)
        .args(["--query", "architecture", "--budget", "4096"])
        .output()
        .unwrap();

    assert!(resolve_1.status.success());
    assert!(resolve_2.status.success());

    assert_eq!(
        resolve_1.stdout, resolve_2.stdout,
        "Rebuild from identical sources must produce byte-identical resolve output"
    );
}

#[test]
fn inspect_output_is_deterministic() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    write_fixture_sources(&sources);

    let cache = tmp.path().join("cache");
    let build = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .unwrap();
    assert!(build.status.success());

    let inspect_a = context_bin()
        .args(["inspect", "--cache"])
        .arg(&cache)
        .output()
        .unwrap();

    let inspect_b = context_bin()
        .args(["inspect", "--cache"])
        .arg(&cache)
        .output()
        .unwrap();

    assert!(inspect_a.status.success());
    assert!(inspect_b.status.success());

    assert_eq!(
        inspect_a.stdout, inspect_b.stdout,
        "Two inspect runs must produce byte-identical output"
    );

    // Verify structure
    let output: serde_json::Value =
        serde_json::from_slice(&inspect_a.stdout).expect("inspect output is not valid JSON");

    assert!(output.get("cache_version").is_some());
    assert!(output.get("document_count").is_some());
    assert!(output.get("total_bytes").is_some());
    assert!(output.get("valid").is_some());
    assert_eq!(output["document_count"].as_u64().unwrap(), 4);
    assert_eq!(output["valid"].as_bool().unwrap(), true);
}

#[test]
fn zero_budget_returns_empty_documents() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    write_fixture_sources(&sources);

    let cache = tmp.path().join("cache");
    let build = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .unwrap();
    assert!(build.status.success());

    let resolve = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache)
        .args(["--query", "deployment", "--budget", "0"])
        .output()
        .unwrap();

    assert!(resolve.status.success());

    let output: serde_json::Value = serde_json::from_slice(&resolve.stdout).unwrap();
    let docs = output["documents"].as_array().unwrap();
    assert!(docs.is_empty(), "Zero budget should select no documents");
}

#[test]
fn empty_query_is_valid() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    write_fixture_sources(&sources);

    let cache = tmp.path().join("cache");
    let build = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .unwrap();
    assert!(build.status.success());

    let resolve = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache)
        .args(["--query", "", "--budget", "4096"])
        .output()
        .unwrap();

    assert!(
        resolve.status.success(),
        "Empty query should succeed: {}",
        String::from_utf8_lossy(&resolve.stderr)
    );

    let output: serde_json::Value = serde_json::from_slice(&resolve.stdout).unwrap();
    assert!(output.get("documents").is_some());
    assert_eq!(output["selection"]["query"].as_str().unwrap(), "");
}
