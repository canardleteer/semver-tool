use assert_cmd::Command;

const TEST_PKG_NAME: &str = "generate";

#[test]
fn cli_validate_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(TEST_PKG_NAME)
        .assert()
        .append_context("validate", "no args")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("-100").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad integer args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("10").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 valid semver arg")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("10").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 valid semver arg")
        .success();
}
