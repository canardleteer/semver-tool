use assert_cmd::Command;

const TEST_PKG_NAME: &str = "filter-test";

#[test]
fn cli_filter_test_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("filter-test")
        .assert()
        .append_context(TEST_PKG_NAME, "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">a.b.c").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver filter arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">1").arg("x.y.z").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg("2.0.0").arg(">1").assert();
    assert
        .append_context(TEST_PKG_NAME, "backwards args")
        .failure();
}

#[test]
fn cli_filter_test_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("filter-test").arg(">1").arg("2.0.0").assert();
    assert.append_context(TEST_PKG_NAME, ">1 test").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("filter-test")
        .arg(">1")
        .arg("0.0.1-rc1.br.0+abc")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, ">1 0.0.1-rc1.br.0+abc")
        .failure();

    // NOTE(canardleteer): I should probably add some more complex filters.
}
