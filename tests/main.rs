//> Chunks of Bytecode tests
//> Scanning on Demand tests-imports
#![allow(non_snake_case)]
use ::std::env;
use ::std::fs;
//< Scanning on Demand tests-imports
use ::std::process::Command;

#[track_caller]
/* Chunks of Bytecode tests < Scanning on Demand tests-run
pub fn run(stdout: &str) {
    let filename = "";
    let expected = (Some(0), stdout.to_owned(), "".to_owned());
*/
//> Scanning on Demand tests-run
pub fn run(testname: &str) {
    const CWD: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/lox");
    env::set_current_dir(CWD).unwrap();
    let filename = testname.replace("__", "/") + ".lox";
    let expected = parse(&filename);
//< Scanning on Demand tests-run
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
/* A Virtual Machine tests < Scanning on Demand tests
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
*/
//> Scanning on Demand tests
fn parse(filename: &str) -> (Option<i32>, String, String) {
    let mut source = fs::read_to_string(&filename).unwrap();
    if !source.ends_with('\n') {
        source.push('\n');
    }
    let mut status = 0;
    let mut output = String::new();
    let mut error = String::new();
    let mut lines = source.split_inclusive('\n');
    let mut line_no = 0;
    'LINES: while let Some(line) = lines.next() {
        line_no += 1;
        if let Some(i) = line.find("// expect: ") {
            output.push_str(&line[i + 11..]);
        } else if line.starts_with("// [c line ") {
            status = 65;
            error.push_str("[");
            error.push_str(&line[6..]);
        } else if let Some(i) = line.find("// [line ") {
            status = 65;
            error.push_str(&line[i + 3..]);
        } else if let Some(i) = line.find("// Error ") {
            status = 65;
            error.push_str(&format!("[line {}] ", line_no));
            error.push_str(&line[i + 3..]);
        } else if let Some(i) = line.find("// expect runtime error: ") {
            status = 70;
            error.push_str(&line[i + 25..]);
            while let Some(line) = lines.next() {
                if line.starts_with("// stack trace") {
                    for line in lines {
                        error.push_str(&line[3..]);
                    }
                    break 'LINES;
                }
            }
            error.push_str(&format!("[line {}] in script\n", line_no));
        }
    }
    (Some(status), output, error)
}
macro_rules! case {
    ($name:ident) => {
        #[test]
        fn $name() { crate::run(stringify!($name)) }
    };
}
use case;
//< Scanning on Demand tests
/* Scanning on Demand tests < Compiling Expressions tests
mod scanning_on_demand {
    use super::case;
    case!(scanning__identifiers);
    case!(scanning__keywords);
    case!(scanning__numbers);
    case!(scanning__punctuators);
    case!(scanning__strings);
    case!(scanning__whitespace);
}
*/
/* Compiling Expressions tests < Global Variables tests
mod compiling_expressions {
    use super::case;
    case!(expressions__evaluate);
}
*/
//> Types of Values tests
//< Types of Values tests
//> Strings tests
//< Strings tests
//> Hash Tables tests
//< Hash Tables tests
//> Global Variables tests
mod global_variables {
    use super::case;
    case!(assignment__associativity);
    case!(assignment__global);
    case!(assignment__grouping);
    case!(assignment__infix_operator);
    case!(assignment__prefix_operator);
    case!(assignment__syntax);
    case!(assignment__undefined);
    case!(bool__equality);
    case!(bool__not);
    case!(comments__line_at_eof);
    case!(comments__only_line_comment);
    case!(comments__only_line_comment_and_line);
    case!(comments__unicode);
    case!(empty_file);
    case!(nil__literal);
    case!(number__leading_dot);
    case!(number__literals);
    case!(number__nan_equality);
    case!(operator__add);
    case!(operator__add_bool_nil);
    case!(operator__add_bool_num);
    case!(operator__add_bool_string);
    case!(operator__add_nil_nil);
    case!(operator__add_num_nil);
    case!(operator__add_string_nil);
    case!(operator__comparison);
    case!(operator__divide);
    case!(operator__divide_nonnum_num);
    case!(operator__divide_num_nonnum);
    case!(operator__equals);
    case!(operator__greater_nonnum_num);
    case!(operator__greater_num_nonnum);
    case!(operator__greater_or_equal_nonnum_num);
    case!(operator__greater_or_equal_num_nonnum);
    case!(operator__less_nonnum_num);
    case!(operator__less_num_nonnum);
    case!(operator__less_or_equal_nonnum_num);
    case!(operator__less_or_equal_num_nonnum);
    case!(operator__multiply);
    case!(operator__multiply_nonnum_num);
    case!(operator__multiply_num_nonnum);
    case!(operator__negate);
    case!(operator__negate_nonnum);
    case!(operator__not_equals);
    case!(operator__subtract);
    case!(operator__subtract_nonnum_num);
    case!(operator__subtract_num_nonnum);
    case!(precedence);
    case!(print__missing_argument);
    case!(string__error_after_multiline);
    case!(string__literals);
    case!(string__multiline);
    case!(string__unterminated);
    case!(variable__redeclare_global);
    case!(variable__redefine_global);
    case!(variable__undefined_global);
    case!(variable__uninitialized);
    case!(variable__use_false_as_var);
    case!(variable__use_global_in_initializer);
    case!(variable__use_nil_as_var);
    case!(variable__use_this_as_var);
}
//< Global Variables tests
