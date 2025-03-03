use assert_cmd::Command;
use proptest::prelude::*;
use proptest_semver::*;
use semver::{BuildMetadata, Version};

const TEST_PKG_NAME: &str = "compare";

#[test]
fn cli_compare_boring_cases() {
    // Error status with no args.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("compare")
        .assert()
        .append_context(TEST_PKG_NAME, "no args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("a.b.c").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("a.b.c").arg("x.y.z").assert();
    assert
        .append_context(TEST_PKG_NAME, "2 bad semver args")
        .failure();
}

/// NOTE(canardleteer): Since these codes are considered unstable for now,
///                     be prepared to make changes in here.
#[test]
fn cli_compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("1.2.3").arg("4.5.6").assert();

    assert
        .append_context(TEST_PKG_NAME, "no exit code reporting")
        .success();

    // Should be (sem: Equal, lex: Equal) aka Success
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("1.2.3")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "exit code reporting")
        .success();

    // Should be (sem: Less, lex: Less) aka 100
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("1.2.3")
        .arg("4.5.6")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "exit code reporting")
        .code(100);

    // Should be (sem: Greater, lex: Greater) aka 122
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("4.5.6")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "exit code reporting")
        .code(122);

    // Should be (sem: Equal, lex: Greater) aka 112
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("1.2.3+1")
        .arg("1.2.3+0")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "exit code reporting")
        .code(112);

    // Should be (sem: Equal, lex: Less) aka 110
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "exit code reporting")
        .code(110);

    // Should be (sem: Equal, lex: Less) aka 110, but overridden by -s
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("-s")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            TEST_PKG_NAME,
            "exit code reporting + semantic equvalence passing",
        )
        .success();

    // Should be (sem: Less, lex: Less) aka 100, and where -s has no impact
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-e")
        .arg("-s")
        .arg("1.2.2")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            TEST_PKG_NAME,
            "exit code reporting + semantic equvalence passing",
        )
        .code(100);

    // These don't match, but should pass anyways since -e is not set
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-s")
        .arg("1.2.4+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            TEST_PKG_NAME,
            "semantic equvalence passing without complex exit code reporting",
        )
        .success();
}

proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        // timeout: 10000,
        cases: 256 * 1,
        .. ProptestConfig::default()
    })]
    #[test]
    fn filter_test_semantic_equal(a in arb_version(), b in arb_version()) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.arg(TEST_PKG_NAME).arg("-s").arg(a.to_string()).arg(b.to_string()).assert();
        assert.append_context(TEST_PKG_NAME, "property test: -s").success();

        // We don't enable `--set-exit-status`, so as long as the input is clean, we should succeed.
    }

    #[test]
    fn filter_test_compare_no_opts(version_a in arb_version(), version_b in arb_version()) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.arg(TEST_PKG_NAME).arg("-s").arg(version_a.to_string()).arg(version_b.to_string()).assert();
        assert.append_context(TEST_PKG_NAME, "property test").success();

        // We don't enable `--set-exit-status`, so as long as the input is clean, we should succeed.
    }

    // NOTE(canardleteer): A more robust test case here, would be for "-s" & "-se"
}
