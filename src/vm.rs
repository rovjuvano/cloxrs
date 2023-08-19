//> A Virtual Machine vm-c
//> Calls and Functions runtime-error-stack
use ::core::iter::*;
//< Calls and Functions runtime-error-stack
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
//> Calls and Functions vm-include-time
use ::std::time::*;
//< Calls and Functions vm-include-time

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
/* A Virtual Machine vm-h < Calls and Functions vm-include-object
pub use crate::chunk::*;
*/
//> Calls and Functions vm-include-object
pub use crate::object::*;
//< Calls and Functions vm-include-object
//> Hash Tables vm-include-table
pub use crate::table::*;
//< Hash Tables vm-include-table
//> vm-include-value
pub use crate::value::*;
//< vm-include-value
//> stack-max

//< stack-max
/* A Virtual Machine stack-max < Calls and Functions frame-max
pub const STACK_MAX: isize = 256;
*/
//> Calls and Functions frame-max
pub const FRAMES_MAX: isize = 64;
pub const STACK_MAX: isize = FRAMES_MAX * UINT8_COUNT;
//< Calls and Functions frame-max
//> Calls and Functions call-frame

#[derive(Clone)] // Copy too but made explicit
pub struct CallFrame {
/* Calls and Functions call-frame < Closures call-frame-closure
    pub function: *mut ObjFunction,
*/
//> Closures call-frame-closure
    pub closure: *mut ObjClosure,
//< Closures call-frame-closure
    pub ip: *mut u8,
    pub slots: *mut Value,
}
//< Calls and Functions call-frame

#[derive(Clone)] // Copy too but made explicit
pub struct VM {
/* A Virtual Machine vm-h < Calls and Functions frame-array
    pub chunk: *mut Chunk,
*/
/* A Virtual Machine ip < Calls and Functions frame-array
    pub ip: *mut u8,
*/
//> Calls and Functions frame-array
    pub frames: [CallFrame; FRAMES_MAX as usize],
    pub frameCount: isize,

//< Calls and Functions frame-array
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
//> Methods and Initializers vm-init-string
    pub initString: *mut ObjString,
//< Methods and Initializers vm-init-string
//> Closures open-upvalues-field
    pub openUpvalues: *mut ObjUpvalue,
//< Closures open-upvalues-field
//> Garbage Collection vm-fields

    pub bytesAllocated: usize,
    pub nextGC: usize,
//< Garbage Collection vm-fields
//> Strings objects-root
    pub objects: *mut Obj,
//< Strings objects-root
//> Garbage Collection vm-gray-stack
    pub grayCount: isize,
    pub grayCapacity: isize,
    pub grayStack: *mut *mut Obj,
//< Garbage Collection vm-gray-stack
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
//> Calls and Functions clock-native
static mut startTime: Instant = unsafe { uninit_static!(Instant) };
fn clockNative(mut _argCount: isize, mut _args: *mut Value) -> Value {
    return NUMBER_VAL!(unsafe { startTime }.elapsed().as_micros() as f64 / 1_000_000.0);
}
//< Calls and Functions clock-native
//> reset-stack
unsafe fn resetStack() {
    unsafe { vm.stackTop = unsafe { &mut vm.stack } as *mut Value };
//> Calls and Functions reset-frame-count
    unsafe { vm.frameCount = 0 };
//< Calls and Functions reset-frame-count
//> Closures init-open-upvalues
    unsafe { vm.openUpvalues = null_mut() };
//< Closures init-open-upvalues
}
//< reset-stack
//> Types of Values runtime-error
unsafe fn runtimeError(mut format: fmt::Arguments<'_>) {
    write!(&mut stderr(), "{}\n", format).unwrap();

/* Types of Values runtime-error < Calls and Functions runtime-error-temp
    let mut instruction: isize = unsafe { vm.ip.offset_from(unsafe { (*vm.chunk).code }) } - 1;
    let mut line: isize = unsafe { *(*vm.chunk).lines.offset(instruction) };
*/
/* Calls and Functions runtime-error-temp < Calls and Functions runtime-error-stack
    let mut frame: *mut CallFrame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize - 1] } as *mut CallFrame;
    let mut instruction: isize = unsafe { (*frame).ip.offset_from(unsafe { (*(*frame).function).chunk.code }) } - 1;
    let mut line: isize = unsafe { *(*(*frame).function).chunk.lines.offset(instruction) };
*/
/* Types of Values runtime-error < Calls and Functions runtime-error-stack
    eprint!("[line {}] in script\n", line);
*/
//> Calls and Functions runtime-error-stack
    for mut i in (0..unsafe { vm.frameCount }).rev() {
        let mut frame: *mut CallFrame = unsafe { &mut vm.frames[i as usize] } as *mut CallFrame;
/* Calls and Functions runtime-error-stack < Closures runtime-error-function
        let mut function: *mut ObjFunction = unsafe { (*frame).function };
*/
//> Closures runtime-error-function
        let mut function: *mut ObjFunction = unsafe { (*(*frame).closure).function };
//< Closures runtime-error-function
        let mut instruction: isize = unsafe { (*frame).ip.offset_from(unsafe { (*function).chunk.code }) } - 1;
        eprint!("[line {}] in ", // [minus]
            unsafe { *(*function).chunk.lines.offset(instruction) });
        if unsafe { (*function).name }.is_null() {
            eprint!("script\n");
        } else {
            eprint!("{}()\n", unsafe { str_from_raw_parts!(unsafe { (*(*function).name).chars }, unsafe { (*(*function).name).length }) });
        }
    }

//< Calls and Functions runtime-error-stack
    unsafe { resetStack() };
}
//< Types of Values runtime-error
//> Calls and Functions define-native
unsafe fn defineNative(mut name: &str, mut function: NativeFn) {
    unsafe { push(OBJ_VAL!(unsafe { copyString(name.as_ptr(), name.len() as isize) })) };
    unsafe { push(OBJ_VAL!(newNative(function))) };
    let _ = unsafe { tableSet(unsafe { &mut vm.globals } as *mut Table,
        unsafe { AS_STRING!(unsafe { vm.stack[0].clone() }) }, unsafe { vm.stack[1].clone() }) };
    let _ = unsafe { pop() };
    let _ = unsafe { pop() };
}
//< Calls and Functions define-native

pub unsafe fn initVM() {
//> call-reset-stack
    unsafe { resetStack() };
//< call-reset-stack
//> Strings init-objects-root
    unsafe { vm.objects = null_mut() };
//< Strings init-objects-root
//> Garbage Collection init-gc-fields
    unsafe { vm.bytesAllocated = 0 };
    unsafe { vm.nextGC = 1024 * 1024 };
//< Garbage Collection init-gc-fields
//> Garbage Collection init-gray-stack

    unsafe { vm.grayCount = 0 };
    unsafe { vm.grayCapacity = 0 };
    unsafe { vm.grayStack = null_mut() };
//< Garbage Collection init-gray-stack
//> Global Variables init-globals

    unsafe { initTable(unsafe { &mut vm.globals } as *mut Table) };
//< Global Variables init-globals
//> Hash Tables init-strings
    unsafe { initTable(unsafe { &mut vm.strings } as *mut Table) };
//< Hash Tables init-strings
//> Methods and Initializers init-init-string

//> null-init-string
    unsafe { vm.initString = null_mut() };
//< null-init-string
    unsafe { vm.initString = unsafe { copyString("init".as_ptr(), 4) } };
//< Methods and Initializers init-init-string
//> Calls and Functions define-native-clock

    unsafe { startTime = Instant::now() };
    unsafe { defineNative("clock", clockNative) };
//< Calls and Functions define-native-clock
}

pub unsafe fn freeVM() {
//> Global Variables free-globals
    unsafe { freeTable(unsafe { &mut vm.globals } as *mut Table) };
//< Global Variables free-globals
//> Hash Tables free-strings
    unsafe { freeTable(unsafe { &mut vm.strings } as *mut Table) };
//< Hash Tables free-strings
//> Methods and Initializers clear-init-string
    unsafe { vm.initString = null_mut() };
//< Methods and Initializers clear-init-string
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
/* Calls and Functions call < Closures call-signature
unsafe fn call(mut function: *mut ObjFunction, mut argCount: isize) -> bool {
*/
//> Calls and Functions call
//> Closures call-signature
unsafe fn call(mut closure: *mut ObjClosure, mut argCount: isize) -> bool {
//< Closures call-signature
/* Calls and Functions check-arity < Closures check-arity
    if argCount != unsafe { (*function).arity } {
        unsafe { runtimeError(format_args!("Expected {} arguments but got {}.",
            unsafe { (*function).arity }, argCount)) };
*/
//> Closures check-arity
    if argCount != unsafe { (*(*closure).function).arity } {
        unsafe { runtimeError(format_args!("Expected {} arguments but got {}.",
            unsafe { (*(*closure).function).arity }, argCount)) };
//< Closures check-arity
//> check-arity
        return false;
    }

//< check-arity
//> check-overflow
    if unsafe { vm.frameCount } == FRAMES_MAX {
        unsafe { runtimeError(format_args!("Stack overflow.")) };
        return false;
    }

//< check-overflow
    let mut frame: *mut CallFrame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize] } as *mut CallFrame;
    unsafe { vm.frameCount += 1 };
/* Calls and Functions call < Closures call-init-closure
    unsafe { (*frame).function = function };
    unsafe { (*frame).ip = unsafe { (*function).chunk.code } };
*/
//> Closures call-init-closure
    unsafe { (*frame).closure = closure };
    unsafe { (*frame).ip = unsafe { (*(*closure).function).chunk.code } };
//< Closures call-init-closure
    unsafe { (*frame).slots = unsafe { vm.stackTop.offset(-argCount - 1) } };
    return true;
}
//< Calls and Functions call
//> Calls and Functions call-value
unsafe fn callValue(mut callee: Value, mut argCount: isize) -> bool {
    if IS_OBJ!(callee) {
        match unsafe { OBJ_TYPE!(callee.clone()) } {
//> Methods and Initializers call-bound-method
            OBJ_BOUND_METHOD => {
                let mut bound: *mut ObjBoundMethod = unsafe { AS_BOUND_METHOD!(callee) };
// //> store-receiver
                unsafe { *vm.stackTop.offset(-argCount - 1) = unsafe { (*bound).receiver.clone() } };
// //< store-receiver
                return unsafe { call(unsafe { (*bound).method }, argCount) };
            }
//< Methods and Initializers call-bound-method
//> Classes and Instances call-class
            OBJ_CLASS => {
                let mut class: *mut ObjClass = unsafe { AS_CLASS!(callee) };
                unsafe { *vm.stackTop.offset(-argCount - 1) = OBJ_VAL!(unsafe { newInstance(class) }) };
//> Methods and Initializers call-init
                let mut initializer: Value = unsafe { uninit::<Value>() };
                if unsafe { tableGet(unsafe { &mut (*class).methods } as *mut Table,
                        unsafe { vm.initString }, &mut initializer as *mut Value) } {
                    return unsafe { call(unsafe { AS_CLOSURE!(initializer) }, argCount) };
//> no-init-arity-error
                } else if argCount != 0 {
                    unsafe { runtimeError(format_args!("Expected 0 arguments but got {}.",
                        argCount)) };
                    return false;
//< no-init-arity-error
                }
//< Methods and Initializers call-init
                return true;
            }
//< Classes and Instances call-class
//> Closures call-value-closure
            OBJ_CLOSURE =>
                return unsafe { call(unsafe { AS_CLOSURE!(callee) }, argCount) },
//< Closures call-value-closure
/* Calls and Functions call-value < Closures call-value-closure
            OBJ_FUNCTION => // [switch]
                return unsafe { call(unsafe { AS_FUNCTION!(callee) }, argCount) },
*/
//> call-native
            OBJ_NATIVE => {
                let mut native: NativeFn = unsafe { AS_NATIVE!(callee) };
                let mut result: Value = unsafe { native(argCount, unsafe { vm.stackTop.offset(-argCount) }) };
                unsafe { vm.stackTop = unsafe { vm.stackTop.offset(-argCount - 1) } };
                unsafe { push(result) };
                return true;
            }
//< call-native
            _ => {} // Non-callable object type.
        }
    }
    unsafe { runtimeError(format_args!("Can only call functions and classes.")) };
    return false;
}
//< Calls and Functions call-value
//> Methods and Initializers invoke-from-class
unsafe fn invokeFromClass(mut class: *mut ObjClass, mut name: *mut ObjString,
        mut argCount: isize) -> bool {
    let mut method: Value = unsafe { uninit::<Value>() };
    if !unsafe { tableGet(unsafe { &mut (*class).methods } as *mut Table, name, &mut method as *mut Value) } {
        unsafe { runtimeError(format_args!("Undefined property '{}'.", unsafe {
            str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) })) };
        return false;
    }
    return unsafe { call(unsafe { AS_CLOSURE!(method) }, argCount) };
}
//< Methods and Initializers invoke-from-class
//> Methods and Initializers invoke
unsafe fn invoke(mut name: *mut ObjString, mut argCount: isize) -> bool {
    let mut receiver: Value = unsafe { peek(argCount) };
//> invoke-check-type

    if !IS_INSTANCE!(receiver) {
        unsafe { runtimeError(format_args!("Only instances have methods.")) };
        return false;
    }

//< invoke-check-type
    let mut instance: *mut ObjInstance = unsafe { AS_INSTANCE!(receiver) };
//> invoke-field

    let mut value: Value = unsafe { uninit::<Value>() };
    if unsafe { tableGet(unsafe { &mut (*instance).fields } as *mut Table, name, &mut value as *mut Value) } {
        unsafe { *(vm.stackTop.offset(-argCount - 1)) = value.clone() };
        return unsafe { callValue(value, argCount) };
    }

//< invoke-field
    return unsafe { invokeFromClass(unsafe { (*instance).class }, name, argCount) };
}
//< Methods and Initializers invoke
//> Methods and Initializers bind-method
unsafe fn bindMethod(mut class: *mut ObjClass, mut name: *mut ObjString) -> bool {
    let mut method: Value = unsafe { uninit::<Value>() };
    if !unsafe { tableGet(unsafe { &mut (*class).methods } as *mut Table, name, &mut method as *mut Value) } {
        unsafe { runtimeError(format_args!("Undefined property '{}'.", unsafe {
            str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) })) };
            return false;
    }

    let mut bound: *mut ObjBoundMethod = unsafe {
        newBoundMethod(unsafe { peek(0) }, unsafe { AS_CLOSURE!(method) }) };
    let _ = unsafe { pop() };
    unsafe { push(OBJ_VAL!(bound)) };
    return true;
}
//< Methods and Initializers bind-method
//> Closures capture-upvalue
unsafe fn captureUpvalue(mut local: *mut Value) -> *mut ObjUpvalue {
//> look-for-existing-upvalue
    let mut prevUpvalue: *mut ObjUpvalue = null_mut();
    let mut upvalue: *mut ObjUpvalue = unsafe { vm.openUpvalues };
    while !upvalue.is_null() && unsafe { (*upvalue).location } > local {
        prevUpvalue = upvalue;
        upvalue = unsafe { (*upvalue).next };
    }

    if !upvalue.is_null() && unsafe { (*upvalue).location } == local {
        return upvalue;
    }

//< look-for-existing-upvalue
    let mut createdUpvalue: *mut ObjUpvalue = unsafe { newUpvalue(local) };
//> insert-upvalue-in-list
    unsafe { (*createdUpvalue).next = upvalue };

    if prevUpvalue.is_null() {
        unsafe { vm.openUpvalues = createdUpvalue };
    } else {
        unsafe { (*prevUpvalue).next = createdUpvalue };
    }

//< insert-upvalue-in-list
    return createdUpvalue;
}
//< Closures capture-upvalue
//> Closures close-upvalues
unsafe fn closeUpvalues(mut last: *mut Value) {
    while !unsafe { vm.openUpvalues }.is_null() &&
            unsafe { (*vm.openUpvalues).location } >= last {
        let mut upvalue: *mut ObjUpvalue = unsafe { vm.openUpvalues };
        unsafe { (*upvalue).closed = unsafe { (*(*upvalue).location).clone() } };
        unsafe { (*upvalue).location = unsafe { &mut (*upvalue).closed } as *mut Value };
        unsafe { vm.openUpvalues = unsafe { (*upvalue).next } };
    }
}
//< Closures close-upvalues
//> Methods and Initializers define-method
unsafe fn defineMethod(mut name: *mut ObjString) {
    let mut method: Value = unsafe { peek(0) };
    let mut class: *mut ObjClass = unsafe { AS_CLASS!(unsafe { peek(1) }) };
    let _ = unsafe { tableSet(unsafe { &mut (*class).methods } as *mut Table, name, method) };
    let _ = unsafe { pop() };
}
//< Methods and Initializers define-method
//> Types of Values is-falsey
fn isFalsey(mut value: Value) -> bool {
    return IS_NIL!(value) || (IS_BOOL!(value) && !unsafe { AS_BOOL!(value) });
}
//< Types of Values is-falsey
//> Strings concatenate
unsafe fn concatenate() {
/* Strings concatenate < Garbage Collection concatenate-peek
    let mut b: *mut ObjString = unsafe { AS_STRING!(unsafe { pop() }) };
    let mut a: *mut ObjString = unsafe { AS_STRING!(unsafe { pop() }) };
*/
//> Garbage Collection concatenate-peek
    let mut b: *mut ObjString = unsafe { AS_STRING!(unsafe { peek(0) }) };
    let mut a: *mut ObjString = unsafe { AS_STRING!(unsafe { peek(1) }) };
//< Garbage Collection concatenate-peek

    let mut length: isize = unsafe { (*a).length } + unsafe { (*b).length };
    let mut chars: *mut u8 = unsafe { ALLOCATE!(u8, (length + 1) as usize) };
    unsafe { copy_nonoverlapping(unsafe { (*a).chars }, chars, unsafe { (*a).length } as usize) };
    unsafe { copy_nonoverlapping(unsafe { (*b).chars }, unsafe { chars.offset(unsafe { (*a).length }) }, unsafe { (*b).length } as usize) };
    unsafe { *chars.offset(length) = b'\0' };

    let mut result: *mut ObjString = unsafe { takeString(chars, length) };
//> Garbage Collection concatenate-pop
    let _ = unsafe { pop() };
    let _ = unsafe { pop() };
//< Garbage Collection concatenate-pop
    unsafe { push(OBJ_VAL!(result)) };
}
//< Strings concatenate
//> run
/* Scanning on Demand vm-interpret-c < Compiling Expressions interpret-chunk
#[allow(dead_code)]
*/
unsafe fn run() -> InterpretResult {
//> Calls and Functions run
    let mut frame: *mut CallFrame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize - 1] } as *mut CallFrame;

/* A Virtual Machine run < Calls and Functions run
    macro_rules! READ_BYTE {
        () => {{
            let mut byte: u8 = unsafe { *vm.ip };
            unsafe { vm.ip = unsafe { vm.ip.offset(1) } };
            byte
        }};
    }
*/
    macro_rules! READ_BYTE {
        () => {{
            let mut byte: u8 = unsafe { *(*frame).ip };
            unsafe { (*frame).ip = unsafe { (*frame).ip.offset(1) } };
            byte
        }};
    }
/* A Virtual Machine read-constant < Calls and Functions run
    macro_rules! READ_CONSTANT {
        () => {{
*/
/* A Virtual Machine read-constant < Types of Values value-type
            unsafe { *(*vm.chunk).constants.values.offset(unsafe { READ_BYTE!() } as isize) }
*/
/* Types of Values value-type < Calls and Functions run
            unsafe { (*(*vm.chunk).constants.values.offset(unsafe { READ_BYTE!() as isize })).clone() }
*/
/* A Virtual Machine read-constant < Calls and Functions run
        }};
    }
*/

/* Jumping Back and Forth read-short < Calls and Functions run
    macro_rules! READ_SHORT {
        () => {{
            let mut short: u16 = ((unsafe { *vm.ip } as u16) << 8) | (unsafe { *vm.ip.offset(1) } as u16);
            unsafe { vm.ip = unsafe { vm.ip.offset(2) } };
            short
        }};
    }
*/
    macro_rules! READ_SHORT {
        () => {{
            let mut short: u16 = ((unsafe { *(*frame).ip } as u16) << 8) | (unsafe { *(*frame).ip.offset(1) } as u16);
            unsafe { (*frame).ip = unsafe { (*frame).ip.offset(2) } };
            short
        }};
    }

/* Calls and Functions run < Closures read-constant
    macro_rules! READ_CONSTANT {
        () => {{
            unsafe { (*(*(*frame).function).chunk.constants.values.offset(unsafe { READ_BYTE!() } as isize)).clone() }
        }};
    }
*/
//> Closures read-constant
    macro_rules! READ_CONSTANT {
        () => {{
            unsafe { (*(*(*(*frame).closure).function).chunk.constants.values.offset(unsafe { READ_BYTE!() } as isize)).clone() }
        }};
    }
//< Closures read-constant

//< Calls and Functions run
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
/* A Virtual Machine trace-execution < Calls and Functions trace-execution
            let _ = unsafe { disassembleInstruction(unsafe { vm.chunk },
                unsafe { vm.ip.offset_from(unsafe { (*vm.chunk).code }) }) };
*/
/* Calls and Functions trace-execution < Closures disassemble-instruction
            let _ = unsafe { disassembleInstruction(
                unsafe { &mut (*(*frame).function).chunk } as *mut Chunk,
                unsafe { (*frame).ip.offset_from(unsafe { (*(*frame).function).chunk.code }) }) };
*/
//> Closures disassemble-instruction
            let _ = unsafe { disassembleInstruction(
                unsafe { &mut (*(*(*frame).closure).function).chunk } as *mut Chunk,
                unsafe { (*frame).ip.offset_from(unsafe { (*(*(*frame).closure).function).chunk.code }) }) };
//< Closures disassemble-instruction
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
/* Local Variables interpret-get-local < Calls and Functions push-local
                unsafe { push(unsafe { vm.stack[slot as usize].clone() }) }; // [slot]
*/
//> Calls and Functions push-local
                unsafe { push(unsafe { (*(*frame).slots.offset(slot as isize)).clone() }) };
//< Calls and Functions push-local
            }
//< Local Variables interpret-get-local
//> Local Variables interpret-set-local
            OP_SET_LOCAL => {
                let mut slot: u8 = unsafe { READ_BYTE!() };
/* Local Variables interpret-set-local < Calls and Functions set-local
                unsafe { vm.stack[slot as usize] = unsafe { peek(0) } };
*/
//> Calls and Functions set-local
                unsafe { *(*frame).slots.offset(slot as isize) = unsafe { peek(0) } };
//< Calls and Functions set-local
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
//> Closures interpret-get-upvalue
            OP_GET_UPVALUE => {
                let mut slot: u8 = unsafe { READ_BYTE!() };
                unsafe { push(unsafe { (*(*(*(*(*frame).closure).upvalues.offset(slot as isize))).location).clone() }) };
            }
//< Closures interpret-get-upvalue
//> Closures interpret-set-upvalue
            OP_SET_UPVALUE => {
                let mut slot: u8 = unsafe { READ_BYTE!() };
                unsafe { *(*(*(*(*frame).closure).upvalues.offset(slot as isize))).location = unsafe { peek(0) } };
            }
//< Closures interpret-set-upvalue
//> Classes and Instances interpret-get-property
            OP_GET_PROPERTY => {
//> get-not-instance
                if !IS_INSTANCE!(unsafe { peek(0) }) {
                    unsafe { runtimeError(format_args!("Only instances have properties.")) };
                    return INTERPRET_RUNTIME_ERROR;
                }

//< get-not-instance
                let mut instance: *mut ObjInstance = unsafe { AS_INSTANCE!(unsafe { peek(0) }) };
                let mut name: *mut ObjString = unsafe { READ_STRING!() };

                let mut value: Value = unsafe { uninit::<Value>() };
                if unsafe { tableGet(unsafe { &mut (*instance).fields } as *mut Table, name, &mut value as *mut Value) } {
                    let _ = unsafe { pop() }; // Instance.
                    unsafe { push(value) };
                    continue;
                }
//> get-undefined

//< get-undefined
/* Classes and Instances get-undefined < Methods and Initializers get-method
                unsafe { runtimeError(format_args!("Undefined property '{}'.", unsafe {
                    str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) })) };
                return INTERPRET_RUNTIME_ERROR;
*/
//> Methods and Initializers get-method
                if !unsafe { bindMethod(unsafe { (*instance).class }, name) } {
                    return INTERPRET_RUNTIME_ERROR;
                }
//< Methods and Initializers get-method
            }
//< Classes and Instances interpret-get-property
//> Classes and Instances interpret-set-property
            OP_SET_PROPERTY => {
//> set-not-instance
                if !IS_INSTANCE!(unsafe { peek(1) }) {
                    unsafe { runtimeError(format_args!("Only instances have fields.")) };
                    return INTERPRET_RUNTIME_ERROR;
                }

//< set-not-instance
                let mut instance: *mut ObjInstance = unsafe { AS_INSTANCE!(unsafe { peek(1) }) };
                let _ = unsafe { tableSet(unsafe { &mut (*instance).fields } as *mut Table, unsafe { READ_STRING!() }, peek(0)) };
                let mut value: Value = unsafe { pop() };
                let _ = unsafe { pop() };
                unsafe { push(value) };
            }
//< Classes and Instances interpret-set-property
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
//> Jumping Back and Forth op-jump
            OP_JUMP => {
                let mut offset: u16 = unsafe { READ_SHORT!() };
/* Jumping Back and Forth op-jump < Calls and Functions jump
                unsafe { vm.ip = unsafe { vm.ip.offset(offset as isize) } };
*/
//> Calls and Functions jump
                unsafe { (*frame).ip = unsafe { (*frame).ip.offset(offset as isize) } };
//< Calls and Functions jump
            }
//< Jumping Back and Forth op-jump
//> Jumping Back and Forth op-jump-if-false
            OP_JUMP_IF_FALSE => {
                let mut offset: u16 = unsafe { READ_SHORT!() };
/* Jumping Back and Forth op-jump-if-false < Calls and Functions jump-if-false
                if isFalsey(unsafe { peek(0) }) { unsafe { vm.ip = unsafe { vm.ip.offset(offset as isize) } }; }
*/
//> Calls and Functions jump-if-false
                if isFalsey(unsafe { peek(0) }) { unsafe { (*frame).ip = unsafe { (*frame).ip.offset(offset as isize) } }; }
//< Calls and Functions jump-if-false
            }
//< Jumping Back and Forth op-jump-if-false
//> Jumping Back and Forth op-loop
            OP_LOOP => {
                let mut offset: u16 = unsafe { READ_SHORT!() };
/* Jumping Back and Forth op-loop < Calls and Functions loop
                unsafe { vm.ip = unsafe { vm.ip.offset(-(offset as isize)) } };
*/
//> Calls and Functions loop
                unsafe { (*frame).ip = unsafe { (*frame).ip.offset(-(offset as isize)) } };
//< Calls and Functions loop
            }
//< Jumping Back and Forth op-loop
//> Calls and Functions interpret-call
            OP_CALL => {
                let mut argCount: isize = unsafe { READ_BYTE!() } as isize;
                if !unsafe { callValue(unsafe { peek(argCount) }, argCount) } {
                    return INTERPRET_RUNTIME_ERROR;
                }
//> update-frame-after-call
                frame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize - 1] } as *mut CallFrame;
//< update-frame-after-call
            }
//< Calls and Functions interpret-call
//> Methods and Initializers interpret-invoke
            OP_INVOKE => {
                let mut method: *mut ObjString = unsafe { READ_STRING!() };
                let mut argCount: isize = unsafe { READ_BYTE!() } as isize;
                if !unsafe { invoke(method, argCount) } {
                    return INTERPRET_RUNTIME_ERROR;
                }
                unsafe { frame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize - 1] } as *mut CallFrame };
            }
//< Methods and Initializers interpret-invoke
//> Closures interpret-closure
            OP_CLOSURE => {
                let mut function: *mut ObjFunction = unsafe { AS_FUNCTION!(unsafe { READ_CONSTANT!() }) };
                let mut closure: *mut ObjClosure = unsafe { newClosure(function) };
                unsafe { push(OBJ_VAL!(closure)) };
//> interpret-capture-upvalues
                for mut i in 0..unsafe { (*closure).upvalueCount } {
                    let mut isLocal: u8 = unsafe { READ_BYTE!() };
                    let mut index: u8 = unsafe { READ_BYTE!() };
                    if isLocal == 1 {
                        unsafe { *(*closure).upvalues.offset(i) = unsafe {
                            captureUpvalue(unsafe { (*frame).slots.offset(index as isize) }) } };
                    } else {
                        unsafe { *(*closure).upvalues.offset(i) = unsafe {
                            *(*(*frame).closure).upvalues.offset(index as isize) } };
                    }
                }
//< interpret-capture-upvalues
            }
//< Closures interpret-closure
//> Closures interpret-close-upvalue
            OP_CLOSE_UPVALUE => {
                unsafe { closeUpvalues(unsafe { vm.stackTop.offset(-1) }) };
                let _ = unsafe { pop() };
            }
//< Closures interpret-close-upvalue
            OP_RETURN => {
/* A Virtual Machine print-return < Global Variables op-return
                unsafe { printValue(unsafe { pop() }) };
                print!("\n");
*/
/* Global Variables op-return < Calls and Functions interpret-return
                // Exit interpreter.
*/
/* A Virtual Machine run < Calls and Functions interpret-return
                return INTERPRET_OK;
*/
//> Calls and Functions interpret-return
                let mut result: Value = unsafe { pop() };
//> Closures return-close-upvalues
                unsafe { closeUpvalues(unsafe { (*frame).slots }) };
//< Closures return-close-upvalues
                unsafe { vm.frameCount -= 1 };
                if unsafe { vm.frameCount == 0 } {
                    let _ = unsafe { pop() };
                    return INTERPRET_OK;
                }

                unsafe { vm.stackTop = unsafe { (*frame).slots } };
                unsafe { push(result) };
                frame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize - 1] } as *mut CallFrame;
//< Calls and Functions interpret-return
            }
//> Classes and Instances interpret-class
            OP_CLASS => {
                unsafe { push(OBJ_VAL!(unsafe { newClass(unsafe { READ_STRING!() }) })) };
            }
//< Classes and Instances interpret-class
//> Methods and Initializers interpret-method
            OP_METHOD => {
                unsafe { defineMethod(unsafe { READ_STRING!() }) };
            }
//< Methods and Initializers interpret-method
        };
    }

// no need to undefine READ_BYTE
//> Jumping Back and Forth undef-read-short
// no need to undefine READ_SHORT
//< Jumping Back and Forth undef-read-short
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
/* Compiling Expressions interpret-chunk < Calls and Functions interpret-stub
    let mut chunk: Chunk = unsafe { uninit::<Chunk>() };
    unsafe { initChunk(&mut chunk as *mut Chunk) };

    if !unsafe { compile(source, &mut chunk as *mut Chunk) } {
        unsafe { freeChunk(&mut chunk as *mut Chunk) };
        return INTERPRET_COMPILE_ERROR;
    }

    unsafe { vm.chunk = &mut chunk as *mut Chunk };
    unsafe { vm.ip = unsafe { (*vm.chunk).code } };
*/
//> Calls and Functions interpret-stub
    let mut function: *mut ObjFunction = unsafe { compile(source) };
    if function.is_null() { return INTERPRET_COMPILE_ERROR; }

    unsafe { push(OBJ_VAL!(function)) };
//< Calls and Functions interpret-stub
/* Calls and Functions interpret-stub < Calls and Functions interpret
    let mut frame: *mut CallFrame = unsafe { &mut vm.frames[unsafe { vm.frameCount } as usize] } as *mut CallFrame;
    unsafe { vm.frameCount += 1 };
    unsafe { (*frame).function = function };
    unsafe { (*frame).ip = unsafe { (*function).chunk.code } };
    unsafe { (*frame).slots = &mut vm.stack as *mut Value };
*/
/* Calls and Functions interpret < Closures interpret
    let _ = unsafe { call(function, 0) };
*/
//> Closures interpret
    let mut closure: *mut ObjClosure = unsafe { newClosure(function) };
    let _ = unsafe { pop() };
    unsafe { push(OBJ_VAL!(closure)) };
    let _ = unsafe { call(closure, 0) };
//< Closures interpret
//< Scanning on Demand vm-interpret-c
//> Compiling Expressions interpret-chunk

/* Compiling Expressions interpret-chunk < Calls and Functions end-interpret
    let mut result: InterpretResult = unsafe { run() };

    unsafe { freeChunk(&mut chunk as *mut Chunk) };
    return result;
*/
//> Calls and Functions end-interpret
    return unsafe { run() };
//< Calls and Functions end-interpret
//< Compiling Expressions interpret-chunk
}
//< interpret
