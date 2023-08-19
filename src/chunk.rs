//> Chunks of Bytecode chunk-c
use ::core::ptr::*;

//> Chunks of Bytecode chunk-h
pub use crate::common::*;
//> chunk-h-include-value
pub use crate::value::*;
//< chunk-h-include-value
//> op-enum

#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum OpCode {
//> op-constant
    OP_CONSTANT,
//< op-constant
    OP_RETURN,
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
pub unsafe fn addConstant(mut chunk: *mut Chunk, mut value: Value) -> isize {
    unsafe { writeValueArray(unsafe { &mut (*chunk).constants } as *mut ValueArray, value) };
    return unsafe { (*chunk).constants.count } - 1;
}
//< add-constant
