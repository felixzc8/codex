#![allow(clippy::unwrap_used, clippy::expect_used)]
use core_test_support::test_codex_exec::test_codex_exec;
use std::path::Path;

#[test]
fn exec_review_uncommitted_succeeds() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("uncommitted")
        .assert()
        .success();
}

#[test]
fn exec_review_uncommitted_with_alias() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("r")
        .arg("uncommitted")
        .assert()
        .success();
}

#[test]
fn exec_review_base_branch_succeeds() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("base-branch")
        .arg("main")
        .assert()
        .success();
}

#[test]
fn exec_review_base_branch_empty_fails() {
    let test = test_codex_exec();

    test.cmd()
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("base-branch")
        .arg("")
        .assert()
        .failure()
        .stderr(::predicates::str::contains("Branch name cannot be empty"));
}

#[test]
fn exec_review_commit_succeeds() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("commit")
        .arg("d5853d9c")
        .assert()
        .success();
}

#[test]
fn exec_review_commit_with_subject() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("commit")
        .arg("d5853d9c")
        .arg("--subject")
        .arg("Fix sandbox command assessment")
        .assert()
        .success();
}

#[test]
fn exec_review_commit_empty_sha_fails() {
    let test = test_codex_exec();

    test.cmd()
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("commit")
        .arg("")
        .assert()
        .failure()
        .stderr(::predicates::str::contains("Commit SHA cannot be empty"));
}

#[test]
fn exec_review_custom_with_prompt() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("custom")
        .arg("Check for security vulnerabilities")
        .assert()
        .success();
}

#[test]
fn exec_review_custom_empty_prompt_fails() {
    let test = test_codex_exec();

    test.cmd()
        .arg("--skip-git-repo-check")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("custom")
        .arg("")
        .assert()
        .failure()
        .stderr(::predicates::str::contains(
            "Custom review prompt cannot be empty",
        ));
}

#[test]
fn exec_review_with_json_flag() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("--json")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("uncommitted")
        .assert()
        .success();
}

#[test]
fn exec_review_respects_global_flags() {
    let test = test_codex_exec();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cli_responses_fixture.sse");

    test.cmd()
        .env("CODEX_RS_SSE_FIXTURE", &fixture)
        .env("OPENAI_BASE_URL", "http://unused.local")
        .arg("--skip-git-repo-check")
        .arg("--model")
        .arg("gpt-4")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("review")
        .arg("uncommitted")
        .assert()
        .success();
}
