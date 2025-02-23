use assert_cmd::Command;

#[test]
fn compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("compare")
        .assert()
        .append_context("compare", "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("compare").arg("--help").assert();
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

/// NOTE(canardleteer): Since these codes are considered unstable for now,
///                     be prepared to make changes in here.
#[test]
fn compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("compare").arg("1.2.3").arg("4.5.6").assert();

    assert
        .append_context("compare", "no exit code reporting")
        .success();

    // Should be (sem: Equal, lex: Equal) aka Success
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3")
        .arg("1.2.3")
        .assert();
    assert
        .append_context("compare", "exit code reporting")
        .success();

    // Should be (sem: Less, lex: Less) aka 100
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3")
        .arg("4.5.6")
        .assert();
    assert
        .append_context("compare", "exit code reporting")
        .code(100);

    // Should be (sem: Greater, lex: Greater) aka 122
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("4.5.6")
        .arg("1.2.3")
        .assert();
    assert
        .append_context("compare", "exit code reporting")
        .code(122);

    // Should be (sem: Equal, lex: Greater) aka 112
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3+1")
        .arg("1.2.3+0")
        .assert();
    assert
        .append_context("compare", "exit code reporting")
        .code(112);

    // Should be (sem: Equal, lex: Less) aka 110
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context("compare", "exit code reporting")
        .code(110);

    // Should be (sem: Equal, lex: Less) aka 110, but overridden by -s
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("-s")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            "compare",
            "exit code reporting + semantic equvalence passing",
        )
        .success();

    // Should be (sem: Less, lex: Less) aka 100, and where -s has no impact
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-e")
        .arg("-s")
        .arg("1.2.2")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            "compare",
            "exit code reporting + semantic equvalence passing",
        )
        .code(100);

    // These don't match, but should pass anyways since -e is not set
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("compare")
        .arg("-s")
        .arg("1.2.4+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            "compare",
            "semantic equvalence passing without complex exit code reporting",
        )
        .success();
}
