//> A Virtual Machine vm-c
use ::core::mem::*;
//> vm-include-stdio
use ::std::*;

//< vm-include-stdio
#[allow(unused_imports)]
use crate::common::*;
//> vm-include-debug
#[cfg_attr(not(DEBUG_TRACE_EXECUTION), allow(unused_imports))]
use crate::debug::*;
//< vm-include-debug
//> A Virtual Machine vm-h
pub use crate::chunk::*;
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
}

//> interpret-result
#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum InterpretResult {
    INTERPRET_OK,
    #[allow(dead_code)]
    INTERPRET_COMPILE_ERROR,
    #[allow(dead_code)]
    INTERPRET_RUNTIME_ERROR,
}
pub use InterpretResult::*;

//< interpret-result
// no need to forward declare initVM
// no need to forward declare freeVM
//> interpret-h
// no need to forward declare interpret
//< interpret-h
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

pub unsafe fn initVM() {
//> call-reset-stack
    unsafe { resetStack() };
//< call-reset-stack
}

pub unsafe fn freeVM() {
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
    return unsafe { *vm.stackTop };
}
//< pop
//> run
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
            unsafe { *(*vm.chunk).constants.values.offset(unsafe { READ_BYTE!() } as isize) }
        }};
    }
//< read-constant
//> binary-op
    macro_rules! BINARY_OP {
        ($op:tt) => {{
            let mut b: f64 = unsafe { pop() };
            let mut a: f64 = unsafe { pop() };
            let mut x: Value = a $op b;
            unsafe { push(x) };
        }};
    }
//< binary-op

    loop {
//> trace-execution
        #[cfg(DEBUG_TRACE_EXECUTION)]
        {
//> trace-stack
            print!("          ");
            let mut slot: *mut Value = unsafe { &mut vm.stack } as *mut Value;
            while slot < unsafe { vm.stackTop } {
                print!("[ ");
                unsafe { printValue(unsafe { *slot }) };
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
//> op-binary
            OP_ADD      => unsafe { BINARY_OP!(+) },
            OP_SUBTRACT => unsafe { BINARY_OP!(-) },
            OP_MULTIPLY => unsafe { BINARY_OP!(*) },
            OP_DIVIDE   => unsafe { BINARY_OP!(/) },
//< op-binary
//> op-negate
            OP_NEGATE   => unsafe { push(-unsafe { pop() }) },
//< op-negate
            OP_RETURN => {
//> print-return
                unsafe { printValue(unsafe { pop() }) };
                print!("\n");
//< print-return
                return INTERPRET_OK;
            }
        };
    }

// no need to undefine READ_BYTE
//> undef-read-constant
// no need to undefine READ_CONSTANT
//< undef-read-constant
//> undef-binary-op
// no need to undefine BINARY_OP
//< undef-binary-op
}
//< run
//> interpret
pub fn interpret(mut chunk: *mut Chunk) -> InterpretResult {
    unsafe { vm.chunk = chunk };
    unsafe { vm.ip = unsafe { (*vm.chunk).code } };
    return unsafe { run() };
}
//< interpret
