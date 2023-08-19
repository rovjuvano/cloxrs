//> Chunks of Bytecode chunk-c
use ::core::ptr::*;

//> Chunks of Bytecode chunk-h
pub use crate::common::*;
//> chunk-h-include-value
pub use crate::value::*;
//< chunk-h-include-value
//> op-enum

/* Scanning on Demand args < Compiling Expressions emit-return
#[allow(dead_code)]
*/
#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum OpCode {
//> op-constant
    OP_CONSTANT,
//< op-constant
//> Types of Values literal-ops
    OP_NIL,
    OP_TRUE,
    OP_FALSE,
//< Types of Values literal-ops
//> Global Variables pop-op
    OP_POP,
//< Global Variables pop-op
//> Local Variables get-local-op
    OP_GET_LOCAL,
//< Local Variables get-local-op
//> Local Variables set-local-op
    OP_SET_LOCAL,
//< Local Variables set-local-op
//> Global Variables get-global-op
    OP_GET_GLOBAL,
//< Global Variables get-global-op
//> Global Variables define-global-op
    OP_DEFINE_GLOBAL,
//< Global Variables define-global-op
//> Global Variables set-global-op
    OP_SET_GLOBAL,
//< Global Variables set-global-op
//> Closures upvalue-ops
    OP_GET_UPVALUE,
    OP_SET_UPVALUE,
//< Closures upvalue-ops
//> Classes and Instances property-ops
    OP_GET_PROPERTY,
    OP_SET_PROPERTY,
//< Classes and Instances property-ops
//> Superclasses get-super-op
    OP_GET_SUPER,
//< Superclasses get-super-op
//> Types of Values comparison-ops
    OP_EQUAL,
    OP_GREATER,
    OP_LESS,
//< Types of Values comparison-ops
//> A Virtual Machine binary-ops
    OP_ADD,
/* A Virtual Machine binary-ops < Compiling Expressions binary
    #[allow(dead_code)]
*/
    OP_SUBTRACT,
/* A Virtual Machine binary-ops < Compiling Expressions binary
    #[allow(dead_code)]
*/
    OP_MULTIPLY,
    OP_DIVIDE,
//> Types of Values not-op
    OP_NOT,
//< Types of Values not-op
//< A Virtual Machine binary-ops
//> A Virtual Machine negate-op
    OP_NEGATE,
//< A Virtual Machine negate-op
//> Global Variables op-print
    OP_PRINT,
//< Global Variables op-print
//> Jumping Back and Forth jump-op
    OP_JUMP,
//< Jumping Back and Forth jump-op
//> Jumping Back and Forth jump-if-false-op
    OP_JUMP_IF_FALSE,
//< Jumping Back and Forth jump-if-false-op
//> Jumping Back and Forth loop-op
    OP_LOOP,
//< Jumping Back and Forth loop-op
//> Calls and Functions op-call
    OP_CALL,
//< Calls and Functions op-call
//> Methods and Initializers invoke-op
    OP_INVOKE,
//< Methods and Initializers invoke-op
//> Superclasses super-invoke-op
    OP_SUPER_INVOKE,
//< Superclasses super-invoke-op
//> Closures closure-op
    OP_CLOSURE,
//< Closures closure-op
//> Closures close-upvalue-op
    OP_CLOSE_UPVALUE,
//< Closures close-upvalue-op
    OP_RETURN,
//> Classes and Instances class-op
    OP_CLASS,
//< Classes and Instances class-op
//> Superclasses inherit-op
    OP_INHERIT,
//< Superclasses inherit-op
//> Methods and Initializers method-op
    OP_METHOD
//< Methods and Initializers method-op
}
pub use OpCode::*;
//< op-enum
//> chunk-struct

#[derive(Clone)] // Copy too but made explicit
pub struct Chunk {
//> count-and-capacity
    pub count: isize,
    pub capacity: isize,
//< count-and-capacity
    pub code: *mut u8,
//> chunk-lines
    pub lines: *mut isize,
//< chunk-lines
//> chunk-constants
    pub constants: ValueArray,
//< chunk-constants
}
//< chunk-struct
//> init-chunk-h

// no need to forward declare initChunk
//< init-chunk-h
//> free-chunk-h
// no need to forward declare freeChunk
//< free-chunk-h
/* Chunks of Bytecode write-chunk-h < Chunks of Bytecode write-chunk-with-line-h
// no need to forward declare writeChunk
*/
//> write-chunk-with-line-h
// no need to forward declare writeChunk
//< write-chunk-with-line-h
//> add-constant-h
// no need to forward declare addConstant
//< add-constant-h
//< Chunks of Bytecode chunk-h
//> chunk-c-include-memory
use crate::memory::*;
//< chunk-c-include-memory
//> Garbage Collection chunk-include-vm
use crate::vm::*;
//< Garbage Collection chunk-include-vm

pub unsafe fn initChunk(mut chunk: *mut Chunk) {
    unsafe { (*chunk).count = 0 };
    unsafe { (*chunk).capacity = 0 };
    unsafe { (*chunk).code = null_mut() };
//> chunk-null-lines
    unsafe { (*chunk).lines = null_mut() };
//< chunk-null-lines
//> chunk-init-constant-array
    unsafe { initValueArray(unsafe { &mut (*chunk).constants } as *mut ValueArray) };
//< chunk-init-constant-array
}
//> free-chunk
/* Scanning on Demand args < Compiling Expressions interpret-chunk
#[allow(dead_code)]
*/
pub unsafe fn freeChunk(mut chunk: *mut Chunk) {
    let _ = unsafe { FREE_ARRAY!(u8, unsafe { (*chunk).code }, unsafe { (*chunk).capacity }) };
//> chunk-free-lines
    let _ = unsafe { FREE_ARRAY!(isize, unsafe { (*chunk).lines } as *mut u8, unsafe { (*chunk).capacity }) };
//< chunk-free-lines
//> chunk-free-constants
    unsafe { freeValueArray(unsafe { &mut (*chunk).constants } as *mut ValueArray) };
//< chunk-free-constants
    unsafe { initChunk(chunk) };
}
//< free-chunk
/* Chunks of Bytecode write-chunk < Chunks of Bytecode write-chunk-with-line
pub unsafe fn writeChunk(mut chunk: *mut Chunk, mut byte: u8) {
*/
//> write-chunk
//> write-chunk-with-line
/* Scanning on Demand args < Compiling Expressions emit-byte
#[allow(dead_code)]
*/
pub unsafe fn writeChunk(mut chunk: *mut Chunk, mut byte: u8, mut line: isize) {
//< write-chunk-with-line
    if unsafe { (*chunk).capacity } < unsafe { (*chunk).count } + 1 {
        let mut oldCapacity: isize = unsafe { (*chunk).capacity };
        unsafe { (*chunk).capacity = GROW_CAPACITY!(oldCapacity) };
        unsafe { (*chunk).code = unsafe { GROW_ARRAY!(u8, unsafe { (*chunk).code }, oldCapacity, unsafe { (*chunk).capacity }) } };
//> write-chunk-line
        unsafe { (*chunk).lines = unsafe { GROW_ARRAY!(isize, unsafe { (*chunk).lines } as *mut u8, oldCapacity, unsafe { (*chunk).capacity }) } };
//< write-chunk-line
    }

    unsafe { *(*chunk).code.offset((*chunk).count) = byte };
//> chunk-write-line
    unsafe { *(*chunk).lines.offset((*chunk).count) = line };
//< chunk-write-line
    unsafe { (*chunk).count += 1 };
}
//< write-chunk
//> add-constant
/* Scanning on Demand args < Compiling Expressions make-constant
#[allow(dead_code)]
*/
pub unsafe fn addConstant(mut chunk: *mut Chunk, mut value: Value) -> isize {
//> Garbage Collection add-constant-push
    unsafe { push(value.clone()) };
//< Garbage Collection add-constant-push
    unsafe { writeValueArray(unsafe { &mut (*chunk).constants } as *mut ValueArray, value) };
//> Garbage Collection add-constant-pop
    let _ = unsafe { pop() };
//< Garbage Collection add-constant-pop
    return unsafe { (*chunk).constants.count } - 1;
}
//< add-constant
