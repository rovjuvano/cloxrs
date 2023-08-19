//> Chunks of Bytecode tests
use ::std::process::Command;

#[track_caller]
pub fn run(stdout: &str) {
    let filename = "";
    let expected = (Some(0), stdout.to_owned(), "".to_owned());
    let cmd = Command::new(env!("CARGO_BIN_EXE_cloxrs"))
        .arg(filename)
        .output().expect("cargo run");
    let stdout = String::from_utf8_lossy(&cmd.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&cmd.stderr).into_owned();
    let actual = (cmd.status.code(), stdout, stderr);
    assert_eq!(actual, expected);
}
#[test]
fn chunks_of_bytecode() {
    run("\
        == test chunk ==\n\
        0000  123 OP_CONSTANT         0 '1.2'\n\
        0002    | OP_RETURN\n\
    ");
}
