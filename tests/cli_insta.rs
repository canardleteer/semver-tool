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
    let insta_targets = vec![
        vec!["filter-test", ">a.b.c"],
        vec!["filter-test", ">1", "x.y.z"],
        vec!["filter-test", "2.0.0", ">1"],
        vec!["filter-test", ">1", "2.0.0"],
        vec!["filter-test", ">1", "0.0.1-rc1.br.0+abc"],
        vec!["sort", "0.1.2-rc0"],
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
