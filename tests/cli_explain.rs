use assert_cmd::Command;

const TEST_PKG_NAME: &str = "explain";

#[test]
fn cli_explain_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("explain")
        .assert()
        .append_context(TEST_PKG_NAME, "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("explain").arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("explain").arg("a.b.c").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver args")
        .failure();
}

#[test]
fn cli_explain_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("explain").arg("0.1.2-rc.0.a.1.b+a.0.b.1").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();
}
