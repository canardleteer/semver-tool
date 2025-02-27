use assert_cmd::Command;

#[test]
fn cli_basics() {
    // Fail with no input.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();

    // Success with --help.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("basic", "help").success();
}
