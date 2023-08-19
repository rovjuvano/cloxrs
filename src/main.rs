//> Chunks of Bytecode main-c
#![no_std]
#![no_implicit_prelude]
#![allow(missing_debug_implementations)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused_mut)]
#![deny(unused_results)]
#![allow(unused_unsafe)]
extern crate alloc;
extern crate std;

//> chunk-h
mod chunk;
//< chunk-h
//> common-h
mod common;
//< common-h
//> debug-h
mod debug;
//< debug-h
//> memory-h
mod memory;
//< memory-h
//> value-h
mod value;
//< value-h

use crate::common::*;
//> main-include-chunk
use crate::chunk::*;
//< main-include-chunk
//> main-include-debug
use crate::debug::*;
//< main-include-debug

fn main() {
//> main-chunk
    let mut chunk: Chunk = unsafe { uninit::<Chunk>() };
    unsafe { initChunk(&mut chunk as *mut Chunk) };
//< main-chunk
//> main-constant

    let mut constant: isize = unsafe { addConstant(&mut chunk as *mut Chunk, 1.2) };
//< main-constant
/* Chunks of Bytecode main-constant < Chunks of Bytecode main-chunk-line
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8) };

*/
//> main-chunk-line
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8, 123) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8, 123) };
//< main-chunk-line
/* Chunks of Bytecode main-chunk < Chunks of Bytecode main-chunk-line
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_RETURN as u8) };
*/
//> main-chunk-line

    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_RETURN as u8, 123) };
//< main-chunk-line
//> main-disassemble-chunk

    unsafe { disassembleChunk(&mut chunk as *mut Chunk, "test chunk") };
//< main-disassemble-chunk
//> main-chunk
    unsafe { freeChunk(&mut chunk as *mut Chunk as *mut Chunk) };
//< main-chunk
}
