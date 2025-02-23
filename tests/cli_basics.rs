use assert_cmd::Command;

#[test]
fn does_it_exist() {
    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"));
    let output = cmd.unwrap();
    println!("{:?}", output);

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("basic", "help").success();
}
