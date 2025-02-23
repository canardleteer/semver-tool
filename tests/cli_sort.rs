use assert_cmd::Command;

#[test]
fn compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().append_context("sort", "no args").failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("sort", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("a.b.c").assert();
    assert.append_context("sort", "1 bad semver args").failure();
}

// NOTE(canardleteer): These are not very robust at all.
#[test]
fn compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("0.1.2-rc0").assert();
    assert.append_context("sort", "single item").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("-f >0")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context("explain", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("-r")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context("explain", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context("explain", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--lexical-sorting")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context("explain", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--lexical-sorting")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context("explain", "help").success();
}
