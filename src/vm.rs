//> A Virtual Machine vm-c
use ::core::mem::*;
//> Strings concatenate
use ::core::ptr::*;
//< Strings concatenate
//> Types of Values include-stdarg
use ::std::io::*;
//< Types of Values include-stdarg
//> vm-include-stdio
use ::std::*;
//> Strings vm-include-string
// no need for additional includes here
//< Strings vm-include-string

//< vm-include-stdio
#[allow(unused_imports)]
use crate::common::*;
//> Scanning on Demand vm-include-compiler
use crate::compiler::*;
//< Scanning on Demand vm-include-compiler
//> vm-include-debug
#[cfg_attr(not(DEBUG_TRACE_EXECUTION), allow(unused_imports))]
use crate::debug::*;
//< vm-include-debug
//> Strings vm-include-object-memory
#[allow(unused_imports)]
use crate::object::*;
use crate::memory::*;
//< Strings vm-include-object-memory
//> A Virtual Machine vm-h
pub use crate::chunk::*;
//> Hash Tables vm-include-table
pub use crate::table::*;
//< Hash Tables vm-include-table
//> vm-include-value
pub use crate::value::*;
//< vm-include-value
//> stack-max

pub const STACK_MAX: isize = 256;
//< stack-max

#[derive(Clone)] // Copy too but made explicit
pub struct VM {
    pub chunk: *mut Chunk,
//> ip
    pub ip: *mut u8,
//< ip
//> vm-stack
    pub stack: [Value; STACK_MAX as usize],
    pub stackTop: *mut Value,
//< vm-stack
//> Global Variables vm-globals
    pub globals: Table,
//< Global Variables vm-globals
//> Hash Tables vm-strings
    pub strings: Table,
//< Hash Tables vm-strings
//> Strings objects-root
    pub objects: *mut Obj,
//< Strings objects-root
}

//> interpret-result
#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum InterpretResult {
    INTERPRET_OK,
/* A Virtual Machine interpret-result < Compiling Expressions interpret-chunk
    #[allow(dead_code)]
*/
    INTERPRET_COMPILE_ERROR,
/* A Virtual Machine interpret-result < Types of Values op-negate
    #[allow(dead_code)]
*/
    INTERPRET_RUNTIME_ERROR,
}
pub use InterpretResult::*;

//< interpret-result
//> Strings extern-vm
// no need to extern vm

//< Strings extern-vm
// no need to forward declare initVM
// no need to forward declare freeVM
/* A Virtual Machine interpret-h < Scanning on Demand vm-interpret-h
// no need to forward declare interpret
*/
//> Scanning on Demand vm-interpret-h
// no need to forward declare interpret
//< Scanning on Demand vm-interpret-h
//> push-pop
// no need to forward declare push
// no need to forward declare pop
//< push-pop
//< A Virtual Machine vm-h

pub static mut vm: VM = unsafe { uninit_static!(VM) }; // [one]
//> reset-stack
unsafe fn resetStack() {
    unsafe { vm.stackTop = unsafe { &mut vm.stack } as *mut Value };
}
//< reset-stack
//> Types of Values runtime-error
unsafe fn runtimeError(mut format: fmt::Arguments<'_>) {
    write!(&mut stderr(), "{}\n", format).unwrap();

    let mut instruction: isize = unsafe { vm.ip.offset_from(unsafe { (*vm.chunk).code }) } - 1;
    let mut line: isize = unsafe { *(*vm.chunk).lines.offset(instruction) };
    eprint!("[line {}] in script\n", line);
    unsafe { resetStack() };
}
//< Types of Values runtime-error

pub unsafe fn initVM() {
//> call-reset-stack
    unsafe { resetStack() };
//< call-reset-stack
//> Strings init-objects-root
    unsafe { vm.objects = null_mut() };
//< Strings init-objects-root
//> Global Variables init-globals

    unsafe { initTable(unsafe { &mut vm.globals } as *mut Table) };
//< Global Variables init-globals
//> Hash Tables init-strings
    unsafe { initTable(unsafe { &mut vm.strings } as *mut Table) };
//< Hash Tables init-strings
}

pub unsafe fn freeVM() {
//> Global Variables free-globals
    unsafe { freeTable(unsafe { &mut vm.globals } as *mut Table) };
//< Global Variables free-globals
//> Hash Tables free-strings
    unsafe { freeTable(unsafe { &mut vm.strings } as *mut Table) };
//< Hash Tables free-strings
//> Strings call-free-objects
    unsafe { freeObjects() };
//< Strings call-free-objects
}
//> push
pub unsafe fn push(mut value: Value) {
    unsafe { *vm.stackTop = value };
    unsafe { vm.stackTop = unsafe { vm.stackTop.offset(1) } };
}
//< push
//> pop
pub unsafe fn pop() -> Value {
    unsafe { vm.stackTop = unsafe { vm.stackTop.offset(-1) } };
/* A Virtual Machine pop < Types of Values value-type
    return unsafe { *vm.stackTop };
*/
//> Types of Values value-type
    return unsafe { (*vm.stackTop).clone() };
//< Types of Values value-type
}
//< pop
//> Types of Values peek
unsafe fn peek(mut distance: isize) -> Value {
    return unsafe { (*vm.stackTop.offset(-1 - distance)).clone() };
}
//< Types of Values peek
//> Types of Values is-falsey
fn isFalsey(mut value: Value) -> bool {
    return IS_NIL!(value) || (IS_BOOL!(value) && !unsafe { AS_BOOL!(value) });
}
//< Types of Values is-falsey
//> Strings concatenate
unsafe fn concatenate() {
    let mut b: *mut ObjString = unsafe { AS_STRING!(unsafe { pop() }) };
    let mut a: *mut ObjString = unsafe { AS_STRING!(unsafe { pop() }) };

    let mut length: isize = unsafe { (*a).length } + unsafe { (*b).length };
    let mut chars: *mut u8 = unsafe { ALLOCATE!(u8, (length + 1) as usize) };
    unsafe { copy_nonoverlapping(unsafe { (*a).chars }, chars, unsafe { (*a).length } as usize) };
    unsafe { copy_nonoverlapping(unsafe { (*b).chars }, unsafe { chars.offset(unsafe { (*a).length }) }, unsafe { (*b).length } as usize) };
    unsafe { *chars.offset(length) = b'\0' };

    let mut result: *mut ObjString = unsafe { takeString(chars, length) };
    unsafe { push(OBJ_VAL!(result)) };
}
//< Strings concatenate
//> run
/* Scanning on Demand vm-interpret-c < Compiling Expressions interpret-chunk
#[allow(dead_code)]
*/
unsafe fn run() -> InterpretResult {
    macro_rules! READ_BYTE {
        () => {{
            let mut byte: u8 = unsafe { *vm.ip };
            unsafe { vm.ip = unsafe { vm.ip.offset(1) } };
            byte
        }};
    }
//> read-constant
    macro_rules! READ_CONSTANT {
        () => {{
//< read-constant
/* A Virtual Machine read-constant < Types of Values value-type
            unsafe { *(*vm.chunk).constants.values.offset(unsafe { READ_BYTE!() } as isize) }
*/
//> Types of Values value-type
            unsafe { (*(*vm.chunk).constants.values.offset(unsafe { READ_BYTE!() as isize })).clone() }
//< Types of Values value-type
//> read-constant
        }};
    }
//< read-constant
//> Global Variables read-string
    macro_rules! READ_STRING {
        () => {{ unsafe { AS_STRING!(unsafe { READ_CONSTANT!() }) } }};
    }
//< Global Variables read-string
/* A Virtual Machine binary-op < Types of Values binary-op
    macro_rules! BINARY_OP {
        ($op:tt) => {{
            let mut b: f64 = unsafe { pop() };
            let mut a: f64 = unsafe { pop() };
            let mut x: Value = a $op b;
            unsafe { push(x) };
        }};
    }
*/
//> Types of Values binary-op
    macro_rules! BINARY_OP {
        ($valueType:ident, $op:tt) => {{
            if !IS_NUMBER!(unsafe { peek(0) }) || !IS_NUMBER!(unsafe { peek(1) }) {
                unsafe { runtimeError(format_args!("Operands must be numbers.")) };
                return INTERPRET_RUNTIME_ERROR;
            }
            let mut b: f64 = unsafe { AS_NUMBER!(unsafe { pop() }) };
            let mut a: f64 = unsafe { AS_NUMBER!(unsafe { pop() }) };
            let mut x: Value = $valueType!(a $op b);
            unsafe { push(x) };
        }};
}
//< Types of Values binary-op

    loop {
//> trace-execution
        #[cfg(DEBUG_TRACE_EXECUTION)]
        {
//> trace-stack
            print!("          ");
            let mut slot: *mut Value = unsafe { &mut vm.stack } as *mut Value;
            while slot < unsafe { vm.stackTop } {
                print!("[ ");
/* A Virtual Machine trace-stack < Types of Values value-type
                unsafe { printValue(unsafe { *slot }) };
*/
//> Types of Values value-type
                unsafe { printValue(unsafe { (*slot).clone() }) };
//< Types of Values value-type
                print!(" ]");
                slot = unsafe { slot.offset(1) };
            }
            print!("\n");
//< trace-stack
            let _ = unsafe { disassembleInstruction(unsafe { vm.chunk },
                unsafe { vm.ip.offset_from(unsafe { (*vm.chunk).code }) }) };
        }

//< trace-execution
        let mut instruction: OpCode = unsafe { transmute(unsafe { READ_BYTE!() }) };
        match instruction {
//> op-constant
            OP_CONSTANT => {
                let mut constant: Value = unsafe { READ_CONSTANT!() };
/* A Virtual Machine op-constant < A Virtual Machine push-constant
                unsafe { printValue(constant) };
                print!("\n");
*/
//> push-constant
                unsafe { push(constant) };
//< push-constant
            }
//< op-constant
//> Types of Values interpret-literals
            OP_NIL => unsafe { push(NIL_VAL!()) },
            OP_TRUE => unsafe { push(BOOL_VAL!(true)) },
            OP_FALSE => unsafe { push(BOOL_VAL!(false)) },
//< Types of Values interpret-literals
//> Global Variables interpret-pop
            OP_POP => { let _ = unsafe { pop() }; }
//< Global Variables interpret-pop
//> Local Variables interpret-get-local
            OP_GET_LOCAL => {
                let mut slot: u8 = unsafe { READ_BYTE!() };
                unsafe { push(unsafe { vm.stack[slot as usize].clone() }) }; // [slot]
            }
//< Local Variables interpret-get-local
//> Local Variables interpret-set-local
            OP_SET_LOCAL => {
                let mut slot: u8 = unsafe { READ_BYTE!() };
                unsafe { vm.stack[slot as usize] = unsafe { peek(0) } };
            }
//< Local Variables interpret-set-local
//> Global Variables interpret-get-global
            OP_GET_GLOBAL => {
                let mut name: *mut ObjString = unsafe { READ_STRING!() };
                let mut value: Value = unsafe { uninit::<Value>() };
                if !unsafe { tableGet(unsafe { &mut vm.globals } as *mut Table, name, &mut value as *mut Value) } {
                    unsafe { runtimeError(format_args!("Undefined variable '{}'.", unsafe {
                        str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) })) };
                    return INTERPRET_RUNTIME_ERROR;
                }
                unsafe { push(value) };
            }
//< Global Variables interpret-get-global
//> Global Variables interpret-define-global
            OP_DEFINE_GLOBAL => {
                let mut name: *mut ObjString = unsafe { READ_STRING!() };
                let _ = unsafe { tableSet(unsafe { &mut vm.globals } as *mut Table, name, unsafe { peek(0) }) };
                let _ = unsafe { pop() };
            }
//< Global Variables interpret-define-global
//> Global Variables interpret-set-global
            OP_SET_GLOBAL => {
                let mut name: *mut ObjString = unsafe { READ_STRING!() };
                if unsafe { tableSet(unsafe { &mut vm.globals } as *mut Table, name, unsafe { peek(0) }) } {
                    let _ = unsafe { tableDelete(unsafe { &mut vm.globals } as *mut Table, name) }; // [delete]
                    unsafe { runtimeError(format_args!("Undefined variable '{}'.", unsafe {
                        str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) })) };
                    return INTERPRET_RUNTIME_ERROR;
                }
            }
//< Global Variables interpret-set-global
//> Types of Values interpret-equal
            OP_EQUAL => {
                let mut b: Value = unsafe { pop() };
                let mut a: Value = unsafe { pop() };
                unsafe { push(BOOL_VAL!(unsafe { valuesEqual(a, b) })) };
            }
//< Types of Values interpret-equal
//> Types of Values interpret-comparison
            OP_GREATER  => unsafe { BINARY_OP!(BOOL_VAL, >) },
            OP_LESS     => unsafe { BINARY_OP!(BOOL_VAL, <) },
//< Types of Values interpret-comparison
/* A Virtual Machine op-binary < Types of Values op-arithmetic
            OP_ADD      => unsafe { BINARY_OP!(+) },
            OP_SUBTRACT => unsafe { BINARY_OP!(-) },
            OP_MULTIPLY => unsafe { BINARY_OP!(*) },
            OP_DIVIDE   => unsafe { BINARY_OP!(/) },
*/
/* A Virtual Machine op-negate < Types of Values op-negate
            OP_NEGATE   => unsafe { push(-unsafe { pop() }) },
*/
/* Types of Values op-arithmetic < Strings add-strings
            OP_ADD      => unsafe { BINARY_OP!(NUMBER_VAL, +) },
*/
//> Strings add-strings
            OP_ADD => {
                if IS_STRING!(unsafe { peek(0) }) && IS_STRING!(unsafe { peek(1) }) {
                    unsafe { concatenate() };
                } else if IS_NUMBER!(unsafe { peek(0) }) && IS_NUMBER!(unsafe { peek(1) }) {
                    let mut b: f64 = unsafe { AS_NUMBER!(unsafe { pop() }) };
                    let mut a: f64 = unsafe { AS_NUMBER!(unsafe { pop() }) };
                    unsafe { push(NUMBER_VAL!(a + b)) };
                } else {
                    unsafe { runtimeError(format_args!(
                        "Operands must be two numbers or two strings.")) };
                    return INTERPRET_RUNTIME_ERROR;
                }
            }
//< Strings add-strings
//> Types of Values op-arithmetic
            OP_SUBTRACT => unsafe { BINARY_OP!(NUMBER_VAL, -) },
            OP_MULTIPLY => unsafe { BINARY_OP!(NUMBER_VAL, *) },
            OP_DIVIDE   => unsafe { BINARY_OP!(NUMBER_VAL, /) },
//< Types of Values op-arithmetic
//> Types of Values op-not
            OP_NOT => {
                unsafe { push(BOOL_VAL!(isFalsey(unsafe { pop() }))) };
            }
//< Types of Values op-not
//> Types of Values op-negate
            OP_NEGATE => {
                if !IS_NUMBER!(unsafe { peek(0) }) {
                    unsafe { runtimeError(format_args!("Operand must be a number.")) };
                    return INTERPRET_RUNTIME_ERROR;
                }
                unsafe { push(NUMBER_VAL!(-unsafe { AS_NUMBER!(unsafe { pop() }) })) };
            }
//< Types of Values op-negate
//> Global Variables interpret-print
            OP_PRINT => {
                unsafe { printValue(unsafe { pop() }) };
                print!("\n");
            }
//< Global Variables interpret-print
            OP_RETURN => {
/* A Virtual Machine print-return < Global Variables op-return
                unsafe { printValue(unsafe { pop() }) };
                print!("\n");
*/
//> Global Variables op-return
                // Exit interpreter.
//< Global Variables op-return
                return INTERPRET_OK;
            }
        };
    }

// no need to undefine READ_BYTE
//> undef-read-constant
// no need to undefine READ_CONSTANT
//< undef-read-constant
//> Global Variables undef-read-string
// no need to undefine READ_STRING
//< Global Variables undef-read-string
//> undef-binary-op
// no need to undefine BINARY_OP
//< undef-binary-op
}
//< run
//> omit
// no need for hack here - use `#[allow(dead_code)]`
//< omit
//> interpret
/* A Virtual Machine interpret < Scanning on Demand vm-interpret-c
pub fn interpret(mut chunk: *mut Chunk) -> InterpretResult {
    unsafe { vm.chunk = chunk };
    unsafe { vm.ip = unsafe { (*vm.chunk).code } };
    return unsafe { run() };
*/
//> Scanning on Demand vm-interpret-c
pub unsafe fn interpret(mut source: *const u8) -> InterpretResult {
/* Scanning on Demand vm-interpret-c < Compiling Expressions interpret-chunk
    unsafe { compile(source) };
    return INTERPRET_OK;
*/
//> Compiling Expressions interpret-chunk
    let mut chunk: Chunk = unsafe { uninit::<Chunk>() };
    unsafe { initChunk(&mut chunk as *mut Chunk) };

    if !unsafe { compile(source, &mut chunk as *mut Chunk) } {
        unsafe { freeChunk(&mut chunk as *mut Chunk) };
        return INTERPRET_COMPILE_ERROR;
    }

    unsafe { vm.chunk = &mut chunk as *mut Chunk };
    unsafe { vm.ip = unsafe { (*vm.chunk).code } };
//< Compiling Expressions interpret-chunk
//< Scanning on Demand vm-interpret-c
//> Compiling Expressions interpret-chunk

    let mut result: InterpretResult = unsafe { run() };

    unsafe { freeChunk(&mut chunk as *mut Chunk) };
    return result;
//< Compiling Expressions interpret-chunk
}
//< interpret
