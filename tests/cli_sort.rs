use assert_cmd::Command;
use proptest::prelude::*;
use proptest_semver::*;
use semver::{Version, VersionReq};

const TEST_PKG_NAME: &str = "sort";

#[test]
fn cli_sort_boring_cases() {
    // Passing status with no args.
    //
    // Why? Because the default behavior with no args is to read from STDIN,
    // and STDIN provides nothing, so the list is truly empty.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(TEST_PKG_NAME)
        .assert()
        .append_context(TEST_PKG_NAME, "no args")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("a.b.c").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver args")
        .failure();
}

#[test]
fn cli_sort_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(TEST_PKG_NAME).arg("0.1.2-rc0").assert();
    assert.append_context(TEST_PKG_NAME, "1 item").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-f >0")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, -f >0")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-f >1")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, -f >1")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-f >a")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, -f >a")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("-r")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, -r")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("--lexical-sorting")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting,")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(TEST_PKG_NAME)
        .arg("--lexical-sorting")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--fail-if-potentially-ambiguous")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("0.1.2+bm0").arg("0.1.2+bm1").assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--fail-if-potentially-ambiguous")
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .success();
}

const SORT_TEST_VERSION_COUNT_SMALL: usize = 32;
const SORT_TEST_VERSION_COUNT_LARGE: usize = 128;

fn sort_test_generic(
    lexical_sorting: bool,
    reverse: bool,
    flatten: bool,
    filter: Option<VersionReq>,
    versions: Vec<Version>,
) {
    let mut args = vec![TEST_PKG_NAME.to_string()];
    if lexical_sorting {
        args.push("--lexical-sorting".to_string());
    }
    if reverse {
        args.push("--reverse".to_string());
    }
    if flatten {
        args.push("--flatten".to_string());
    }

    if let Some(filter) = filter {
        args.push(format!("--filter"));
        args.push(format!("{}", filter));
    }

    args.append(
        &mut versions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.args(&args).assert();
    assert.append_context(TEST_PKG_NAME, "prop test").success();
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
    // Using some large number of filters is unlikely to provide us with half the
    // test cases,
    #[test]
    fn sort_test_small(lexical_sorting: bool, reverse: bool, flatten: bool, filter in arb_optional_version_req(0.5, 2), versions in arb_vec_versions(SORT_TEST_VERSION_COUNT_SMALL)) {
        sort_test_generic(lexical_sorting, reverse, flatten, filter, versions);
    }

    // Since the filters are incredibly complex from the framework, the odds of
    // not making mutually exclusinve comparators is small. We're just testing
    // huge input here.
    #[test]
    fn sort_test_large(lexical_sorting: bool, reverse: bool, flatten: bool, filter in arb_optional_version_req(0.5, 2), versions in arb_vec_versions(SORT_TEST_VERSION_COUNT_LARGE)) {
        sort_test_generic(lexical_sorting, reverse, flatten, filter, versions);
    }
}
