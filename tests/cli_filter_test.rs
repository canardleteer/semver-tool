use assert_cmd::Command;

#[test]
fn compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .append_context("filter-test", "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("filter-test", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">a.b.c").assert();
    assert
        .append_context("filter-test", "1 bad semver filter arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">1").arg("x.y.z").assert();
    assert
        .append_context("filter-test", "1 bad semver arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg("2.0.0").arg(">1").assert();
    assert
        .append_context("filter-test", "backwards args")
        .failure();
}

#[test]
fn compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">1").arg("2.0.0").assert();
    assert.append_context("filter-test", ">1 test").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("filter-test")
        .arg(">1")
        .arg("0.0.1-rc1.br.0+abc")
        .assert();
    assert
        .append_context("filter-test", ">1 0.0.1-rc1.br.0+abc")
        .failure();

    // NOTE(canardleteer): I should probably add some more complex filters.
}
