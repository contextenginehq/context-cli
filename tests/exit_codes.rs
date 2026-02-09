//! Exit code tests for the context CLI.
//!
//! Validates that the CLI returns the correct frozen exit codes
//! per cli_spec.md for various error conditions.

use std::fs;
use std::process::Command;

fn context_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_context"))
}

// Exit code constants (frozen per cli_spec.md)
const CACHE_MISSING: i32 = 4;
const CACHE_INVALID: i32 = 5;
const IO_ERROR: i32 = 6;

#[test]
fn missing_cache_returns_exit_code_4() {
    let tmp = tempfile::tempdir().unwrap();
    let nonexistent = tmp.path().join("does-not-exist");

    let output = context_bin()
        .args(["resolve", "--cache"])
        .arg(&nonexistent)
        .args(["--query", "test", "--budget", "1000"])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code().unwrap(),
        CACHE_MISSING,
        "Missing cache should return exit code {}. stderr: {}",
        CACHE_MISSING,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn invalid_manifest_returns_exit_code_5() {
    let tmp = tempfile::tempdir().unwrap();
    let cache_dir = tmp.path().join("cache");
    fs::create_dir_all(&cache_dir).unwrap();

    // Write an invalid manifest
    fs::write(cache_dir.join("manifest.json"), "not valid json").unwrap();

    let output = context_bin()
        .args(["resolve", "--cache"])
        .arg(&cache_dir)
        .args(["--query", "test", "--budget", "1000"])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code().unwrap(),
        CACHE_INVALID,
        "Invalid manifest should return exit code {}. stderr: {}",
        CACHE_INVALID,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn inspect_missing_cache_returns_exit_code_4() {
    let tmp = tempfile::tempdir().unwrap();
    let nonexistent = tmp.path().join("does-not-exist");

    let output = context_bin()
        .args(["inspect", "--cache"])
        .arg(&nonexistent)
        .output()
        .unwrap();

    assert_eq!(
        output.status.code().unwrap(),
        CACHE_MISSING,
        "Inspect with missing cache should return exit code {}. stderr: {}",
        CACHE_MISSING,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn inspect_invalid_manifest_returns_exit_code_5() {
    let tmp = tempfile::tempdir().unwrap();
    let cache_dir = tmp.path().join("cache");
    fs::create_dir_all(&cache_dir).unwrap();

    fs::write(cache_dir.join("manifest.json"), "{broken").unwrap();

    let output = context_bin()
        .args(["inspect", "--cache"])
        .arg(&cache_dir)
        .output()
        .unwrap();

    assert_eq!(
        output.status.code().unwrap(),
        CACHE_INVALID,
        "Inspect with invalid manifest should return exit code {}. stderr: {}",
        CACHE_INVALID,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn build_nonexistent_sources_returns_exit_code_6() {
    let tmp = tempfile::tempdir().unwrap();
    let nonexistent = tmp.path().join("no-such-dir");

    let output = context_bin()
        .args(["build", "--sources"])
        .arg(&nonexistent)
        .arg("--cache")
        .arg(tmp.path().join("cache"))
        .output()
        .unwrap();

    assert_eq!(
        output.status.code().unwrap(),
        IO_ERROR,
        "Build with nonexistent sources should return exit code {}. stderr: {}",
        IO_ERROR,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn build_force_overwrites_existing_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let sources = tmp.path().join("sources");
    let docs = sources.join("docs");
    fs::create_dir_all(&docs).unwrap();
    fs::write(docs.join("test.md"), "Test content for building").unwrap();

    let cache = tmp.path().join("cache");

    // Build once
    let first = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .unwrap();
    assert!(first.status.success(), "First build should succeed");

    // Build again without --force should fail (output exists)
    let second = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .output()
        .unwrap();
    assert!(
        !second.status.success(),
        "Second build without --force should fail"
    );

    // Build with --force should succeed
    let forced = context_bin()
        .args(["build", "--sources"])
        .arg(&sources)
        .arg("--cache")
        .arg(&cache)
        .arg("--force")
        .output()
        .unwrap();
    assert!(
        forced.status.success(),
        "Build with --force should succeed: {}",
        String::from_utf8_lossy(&forced.stderr)
    );
}

#[test]
fn stderr_contains_error_prefix() {
    let tmp = tempfile::tempdir().unwrap();
    let nonexistent = tmp.path().join("nope");

    let output = context_bin()
        .args(["resolve", "--cache"])
        .arg(&nonexistent)
        .args(["--query", "test", "--budget", "1000"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with("error:"),
        "Error output should start with 'error:' prefix, got: {}",
        stderr
    );
}
