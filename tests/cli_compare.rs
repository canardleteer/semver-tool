use assert_cmd::Command;

#[test]
fn compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().append_context("compare", "no args").failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("compare", "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("compare").arg("a.b.c").assert();
    assert
        .append_context("compare", "1 bad semver args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("compare").arg("a.b.c").arg("x.y.z").assert();
    assert
        .append_context("compare", "2 bad semver args")
        .failure();
}

#[test]
fn compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("compare").arg("1.2.3").arg("4.5.6").assert();

    assert
        .append_context("compare", "no exit code reporting")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3")
        .arg("1.2.3")
        .assert();
    // Should be (sem: Equal, lex: Equal) aka Success
    assert
        .append_context("compare", "exit code reporting")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3")
        .arg("4.5.6")
        .assert();
    // Should be (sem: Less, lex: Less) aka 100
    assert
        .append_context("compare", "exit code reporting")
        .code(100);

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("4.5.6")
        .arg("1.2.3")
        .assert();
    // Should be (sem: Greater, lex: Greater) aka 122
    assert
        .append_context("compare", "exit code reporting")
        .code(122);

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3+1")
        .arg("1.2.3+0")
        .assert();
    // Should be (sem: Equal, lex: Greater) aka 112
    assert
        .append_context("compare", "exit code reporting")
        .code(112);

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    // Should be (sem: Equal, lex: Less) aka 110
    assert
        .append_context("compare", "exit code reporting")
        .code(110);
}
