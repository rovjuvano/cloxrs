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
/* Chunks of Bytecode tests < A Virtual Machine tests
#[test]
fn chunks_of_bytecode() {
    run("\
        == test chunk ==\n\
        0000  123 OP_CONSTANT         0 '1.2'\n\
        0002    | OP_RETURN\n\
    ");
}
*/
//> A Virtual Machine tests
#[test]
fn a_virtual_machine() {
    run("\
        == test chunk ==\n\
        0000  123 OP_CONSTANT         0 '1.2'\n\
        0002    | OP_CONSTANT         1 '3.4'\n\
        0004    | OP_ADD\n\
        0005    | OP_CONSTANT         2 '5.6'\n\
        0007    | OP_DIVIDE\n\
        0008    | OP_NEGATE\n\
        0009    | OP_RETURN\n\
        -0.8214285714285714\n\
    ");
}
//< A Virtual Machine tests
