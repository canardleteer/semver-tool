use assert_cmd::Command;
use proptest::prelude::*;
use proptest_semver::*;

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

fn filter_test_generic(filter: semver::VersionReq, version: semver::Version) {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("filter-test")
        .arg(filter.to_string())
        .arg(version.to_string())
        .assert();
    let res = assert
        .append_context(TEST_PKG_NAME, "property test")
        .try_success();

    // It doesn't matter what our status is really, just that it aligns with what we expect.
    match res {
        Ok(_) => {
            if !filter.matches(&version) {
                panic!("the cli succeeded, when it should have failed.");
            }
        }
        Err(_) => {
            if filter.matches(&version) {
                panic!("the cli failed, when it should have succeeded.");
            }
        }
    }
}

const FILTER_TEST_COMPARATOR_LENGTH_LARGE: usize = MAX_COMPARATORS_IN_VERSION_REQ_STRING;
proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        // timeout: 10000,
        cases: 256 * 1,
        .. ProptestConfig::default()
    })]
    // Using some large number of filters is unlikely to provide us with half the
    // test cases,
    #[test]
    fn filter_test_small(filter in arb_version_req(1), version in arb_version()) {
        filter_test_generic(filter, version);
    }

    // Since the filters are incredibly complex from the framework, the odds of
    // not making mutually exclusinve comparators is small. We're just testing
    // huge input here.
    #[test]
    fn filter_test_large(filter in arb_version_req(FILTER_TEST_COMPARATOR_LENGTH_LARGE), version in arb_version()) {
        filter_test_generic(filter, version);
    }
}
