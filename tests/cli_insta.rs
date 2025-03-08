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
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn cli() -> Command {
    Command::new(get_cargo_bin("sem-tool"))
}

#[test]
fn cli_insta() {
    // NOTE(canardleteer): This should be a map with snapshot names.
    //
    //                     Until then, if these get reordered, after
    //                     confirming correctness, you'll need to use
    //                     `INSTA_UPDATE=always` locally to reset snapshots.
    //
    // NOTE(canardleteer): No `generate` tests yet.
    let insta_targets = vec![
        vec!["filter-test", ">a.b.c"],
        vec!["filter-test", ">1", "x.y.z"],
        vec!["filter-test", "2.0.0", ">1"],
        vec!["filter-test", ">1", "2.0.0"],
        vec!["filter-test", ">1", "0.0.1-rc1.br.0+abc"],
        vec!["sort", "0.1.2-rc0"],
        vec![
            "sort",
            "--lexical-sorting",
            "--fail-if-potentially-ambiguous",
            "0.1.2+bm0",
            "0.1.2+bm1",
        ],
        vec!["sort", "--lexical-sorting", "0.1.2+bm0", "0.1.2+bm1"],
        vec!["sort", "-r", "0.1.2-rc0", "0.1.2-rc1"],
        vec!["sort", "--flatten", "0.1.2-rc0", "0.1.2-rc1"],
        vec!["sort", "--lexical-sorting", "0.1.2-rc0", "0.1.2-rc1"],
        vec![
            "sort",
            "--lexical-sorting",
            "--flatten",
            "0.1.2-rc0",
            "0.1.2-rc1",
        ],
        vec!["sort", "-f", ">1", "0.1.2-rc0", "0.1.2-rc1"],
        vec!["sort", "-f", ">0", "0.1.2-rc0", "0.1.2-rc1"],
        vec!["sort", "-f", ">a", "0.1.2-rc0", "0.1.2-rc1"],
        vec!["validate", "a.b.c"],
        vec!["validate", "0.1.2-rc.0.a.1.b+a.0.b.1"],
        vec!["validate", "-s", "18446744073709551616.0.0"],
        vec!["validate", "18446744073709551616.0.0"],
        vec!["explain", "a.b.c"],
        vec!["explain", "0.1.2-rc.0.a.1.b+a.0.b.1"],
        vec!["compare", "1.2.3", "4.5.6"],
        vec!["compare", "-e", "1.2.3", "1.2.3"],
        vec!["compare", "-e", "1.2.3", "4.5.6"],
        vec!["compare", "-e", "4.5.6", "1.2.3"],
        vec!["compare", "-e", "1.2.3+1", "1.2.3+0"],
        vec!["compare", "-e", "1.2.3+0", "1.2.3+1"],
        vec!["compare", "-e", "-s", "1.2.3+0", "1.2.3+1"],
        vec!["compare", "-e", "-s", "1.2.2", "1.2.3+1"],
        vec!["compare", "-s", "1.2.4+0", "1.2.3+1"],
    ];

    for args in insta_targets {
        assert_cmd_snapshot!(cli().args(args));
    }
}
