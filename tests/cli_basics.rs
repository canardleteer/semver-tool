use assert_cmd::Command;

#[test]
fn does_it_exist() {
    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"));
    let output = cmd.unwrap();
    println!("{:?}", output);
}
