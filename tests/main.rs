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
//> Local Variables tests
mod local_variables {
    use super::case;
    case!(assignment__local);
    case!(block__scope);
    case!(variable__duplicate_local);
    case!(variable__in_middle_of_block);
    case!(variable__in_nested_block);
    case!(variable__scope_reuse_in_different_blocks);
    case!(variable__shadow_and_local);
    case!(variable__shadow_global);
    case!(variable__shadow_local);
    case!(variable__undefined_local);
    case!(variable__use_local_in_initializer);
}
//< Local Variables tests
//> Jumping Back and Forth tests
mod jumping_back_and_forth {
    use super::case;
    case!(block__empty);
    case!(for__class_in_body);
    case!(for__fun_in_body);
    case!(for__scope);
    case!(for__statement_condition);
    case!(for__statement_increment);
    case!(for__statement_initializer);
    case!(for__var_in_body);
    case!(if__class_in_else);
    case!(if__class_in_then);
    case!(if__dangling_else);
    case!(if__else);
    case!(if__fun_in_else);
    case!(if__fun_in_then);
    case!(if__if);
    case!(if__truth);
    case!(if__var_in_else);
    case!(if__var_in_then);
    case!(limit__loop_too_large);
    case!(logical_operator__and);
    case!(logical_operator__and_truth);
    case!(logical_operator__or);
    case!(logical_operator__or_truth);
    case!(variable__unreached_undefined);
    case!(while__class_in_body);
    case!(while__fun_in_body);
    case!(while__syntax);
    case!(while__var_in_body);
}
//< Jumping Back and Forth tests
//> Calls and Functions tests
mod calls_and_functions {
    use super::case;
    case!(call__bool);
    case!(call__nil);
    case!(call__num);
    case!(call__string);
    case!(for__return_inside);
    case!(for__syntax);
    case!(function__body_must_be_block);
    case!(function__empty_body);
    case!(function__extra_arguments);
    case!(function__local_mutual_recursion);
    case!(function__missing_arguments);
    case!(function__missing_comma_in_parameters);
    case!(function__mutual_recursion);
    case!(function__nested_call_with_arguments);
    case!(function__parameters);
    case!(function__print);
    case!(function__recursion);
    case!(function__too_many_arguments);
    case!(function__too_many_parameters);
    case!(limit__no_reuse_constants);
    case!(limit__stack_overflow);
    case!(limit__too_many_constants);
    case!(limit__too_many_locals);
    case!(return__after_else);
    case!(return__after_if);
    case!(return__after_while);
    case!(return__at_top_level);
    case!(return__in_function);
    case!(return__return_nil_if_no_value);
    case!(unexpected_character);
    case!(variable__collide_with_parameter);
    case!(variable__duplicate_parameter);
    case!(variable__early_bound);
    case!(while__return_inside);
}
//< Calls and Functions tests
//> Closures tests
mod closures {
    use super::case;
    case!(closure__assign_to_closure);
    case!(closure__assign_to_shadowed_later);
    case!(closure__close_over_function_parameter);
    case!(closure__close_over_later_variable);
    case!(closure__closed_closure_in_function);
    case!(closure__nested_closure);
    case!(closure__open_closure_in_function);
    case!(closure__reference_closure_multiple_times);
    case!(closure__reuse_closure_slot);
    case!(closure__shadow_closure_with_local);
    case!(closure__unused_closure);
    case!(closure__unused_later_closure);
    case!(for__closure_in_body);
    case!(for__return_closure);
    case!(function__local_recursion);
    case!(limit__too_many_upvalues);
    case!(regression__40);
    case!(while__closure_in_body);
    case!(while__return_closure);
}
//< Closures tests
//> Garbage Collection tests
//< Garbage Collection tests
//> Classes and Instances tests
mod classes_and_instances {
    use super::case;
    case!(call__object);
    case!(class__empty);
    case!(field__call_function_field);
    case!(field__call_nonfunction_field);
    case!(field__get_on_bool);
    case!(field__get_on_class);
    case!(field__get_on_function);
    case!(field__get_on_nil);
    case!(field__get_on_num);
    case!(field__get_on_string);
    case!(field__many);
    case!(field__on_instance);
    case!(field__set_evaluation_order);
    case!(field__set_on_bool);
    case!(field__set_on_class);
    case!(field__set_on_function);
    case!(field__set_on_nil);
    case!(field__set_on_num);
    case!(field__set_on_string);
    case!(field__undefined);
    case!(number__decimal_point_at_eof);
    case!(number__trailing_dot);
    case!(operator__not);
    case!(operator__not_class);
}
//< Classes and Instances tests
//> Methods and Initializers tests
mod methods_and_initializers {
    use super::case;
    case!(assignment__to_this);
    case!(class__local_reference_self);
    case!(class__reference_self);
    case!(closure__close_over_method_parameter);
    case!(constructor__arguments);
    case!(constructor__call_init_early_return);
    case!(constructor__call_init_explicitly);
    case!(constructor__default);
    case!(constructor__default_arguments);
    case!(constructor__early_return);
    case!(constructor__extra_arguments);
    case!(constructor__init_not_method);
    case!(constructor__missing_arguments);
    case!(constructor__return_in_nested_function);
    case!(constructor__return_value);
    case!(field__get_and_set_method);
    case!(field__method);
    case!(field__method_binds_this);
    case!(method__arity);
    case!(method__empty_block);
    case!(method__extra_arguments);
    case!(method__missing_arguments);
    case!(method__not_found);
    case!(method__print_bound_method);
    case!(method__refer_to_name);
    case!(method__too_many_arguments);
    case!(method__too_many_parameters);
    case!(operator__equals_class);
    case!(operator__equals_method);
    case!(return__in_method);
    case!(this__closure);
    case!(this__nested_class);
    case!(this__nested_closure);
    case!(this__this_at_top_level);
    case!(this__this_in_method);
    case!(this__this_in_top_level_function);
    case!(variable__local_from_method);
}
//< Methods and Initializers tests
//> Superclasses tests
mod superclasses {
    use super::case;
    case!(class__inherit_self);
    case!(class__inherited_method);
    case!(class__local_inherit_other);
    case!(class__local_inherit_self);
    case!(inheritance__constructor);
    case!(inheritance__inherit_from_function);
    case!(inheritance__inherit_from_nil);
    case!(inheritance__inherit_from_number);
    case!(inheritance__inherit_methods);
    case!(inheritance__parenthesized_superclass);
    case!(inheritance__set_fields_from_base_class);
    case!(regression__394);
    case!(super__bound_method);
    case!(super__call_other_method);
    case!(super__call_same_method);
    case!(super__closure);
    case!(super__constructor);
    case!(super__extra_arguments);
    case!(super__indirectly_inherited);
    case!(super__missing_arguments);
    case!(super__no_superclass_bind);
    case!(super__no_superclass_call);
    case!(super__no_superclass_method);
    case!(super__parenthesized);
    case!(super__reassign_superclass);
    case!(super__super_at_top_level);
    case!(super__super_in_closure_in_inherited_method);
    case!(super__super_in_inherited_method);
    case!(super__super_in_top_level_function);
    case!(super__super_without_dot);
    case!(super__super_without_name);
    case!(super__this_in_superclass_method);
}
//< Superclasses tests
//> Optimization tests
// benchmark/binary_trees.lox
// benchmark/equality.lox
// benchmark/fib.lox
// benchmark/instantiation.lox
// benchmark/invocation.lox
// benchmark/method_call.lox
// benchmark/properties.lox
// benchmark/string_equality.lox
// benchmark/trees.lox
// benchmark/zoo.lox
// benchmark/zoo_batch.lox
//< Optimization tests
