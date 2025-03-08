//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2025 canardleteer
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
use assert_cmd::Command;

const TEST_PKG_NAME: &str = "basic";

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
    assert.append_context(TEST_PKG_NAME, "help").success();
}
