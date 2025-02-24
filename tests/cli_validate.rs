use assert_cmd::Command;

#[test]
fn compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("validate")
        .assert()
        .append_context("validate", "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("validate").arg("--help").assert();
    assert.append_context("validate", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("validate").arg("a.b.c").assert();
    assert
        .append_context("validate", "1 bad semver args")
        .failure();
}

#[test]
fn compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("validate").arg("0.1.2-rc.0.a.1.b+a.0.b.1").assert();
    assert.append_context("validate", "help").success();
}
