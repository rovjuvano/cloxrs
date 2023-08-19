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
//> Scanning on Demand compiler-h
mod compiler;
//< Scanning on Demand compiler-h
//> debug-h
mod debug;
//< debug-h
//> memory-h
mod memory;
//< memory-h
//> Strings object-h
mod object;
//< Strings object-h
//> Scanning on Demand scanner-h
mod scanner;
//< Scanning on Demand scanner-h
//> value-h
mod value;
//< value-h
//> A Virtual Machine vm-h
mod vm;
//< A Virtual Machine vm-h
//> Scanning on Demand main-includes

use ::alloc::alloc::*;
use ::alloc::string::*;
use ::alloc::vec::*;
use ::core::iter::*;
use ::core::mem::*;
use ::core::result::Result::*;
use ::std::*;
use ::std::env::*;
use ::std::fs::*;
use ::std::io::*;
use ::std::process::*;
//< Scanning on Demand main-includes

use crate::common::*;
//> main-include-chunk
//> Scanning on Demand args
#[allow(unused_imports)]
//< Scanning on Demand args
use crate::chunk::*;
//< main-include-chunk
//> main-include-debug
//> Scanning on Demand args
#[allow(unused_imports)]
//< Scanning on Demand args
use crate::debug::*;
//< main-include-debug
//> A Virtual Machine main-include-vm
use crate::vm::*;
//< A Virtual Machine main-include-vm
//> Scanning on Demand repl

fn repl() {
    let mut line: String = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        line.clear();
        if let Ok(0) = stdin().read_line(&mut line) {
            print!("\n");
            break;
        }

        line.push('\0');
        let _ = unsafe { interpret(line.as_ptr()) };
    }
}
//< Scanning on Demand repl
//> Scanning on Demand read-file
fn readFile(mut path: &str) -> *mut u8 {
    let mut file: io::Result<File> = File::open(path);
//> no-file
    let mut file: File = match file {
        Ok(x) => x,
        Err(_) => {
            eprint!("Could not open file \"{}\".\n", path);
            exit(74);
        }
    };
//< no-file

    let mut fileSize: u64 = file.seek(SeekFrom::End(0)).unwrap();
    file.rewind().unwrap();

    let mut layout: Layout = Layout::array::<u8>((fileSize + 1) as usize).unwrap();
    let mut buffer: *mut u8 = unsafe { alloc(layout) };
//> no-buffer
    if buffer.is_null() {
        eprint!("Not enough memory to read \"{}\".\n", path);
        exit(74);
    }

//< no-buffer
    let mut bytesRead: usize = file.read(unsafe { slice::from_raw_parts_mut(buffer, fileSize as usize) }).unwrap();
//> no-read
    if (bytesRead as u64) < fileSize {
        eprint!("Could not read file \"%{}\".\n", path);
        exit(74);
    }

//< no-read
    unsafe { *buffer.add(bytesRead) = b'\0' };

    drop(file);
    buffer
}
//< Scanning on Demand read-file
//> Scanning on Demand run-file
fn runFile(mut path: &str) {
    let mut source: *mut u8 = readFile(path);
    let mut result: InterpretResult = unsafe { interpret(source as *const u8) };
    unsafe { // [owner]
        dealloc(source, Layout::array::<u8>(unsafe { strlen(source as *const c_char) }).unwrap());
    };

    match result {
        INTERPRET_OK => {},
        INTERPRET_COMPILE_ERROR => exit(65),
        INTERPRET_RUNTIME_ERROR => exit(70),
    };
}
//< Scanning on Demand run-file

fn main() {
//> A Virtual Machine main-init-vm
    unsafe { initVM() };

//< A Virtual Machine main-init-vm
/* Chunks of Bytecode main-chunk < Scanning on Demand args
    let mut chunk: Chunk = unsafe { uninit::<Chunk>() };
    unsafe { initChunk(&mut chunk as *mut Chunk) };
*/
/* Chunks of Bytecode main-constant < Scanning on Demand args

    let mut constant: isize = unsafe { addConstant(&mut chunk as *mut Chunk, 1.2) };
*/
/* Chunks of Bytecode main-constant < Chunks of Bytecode main-chunk-line
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8) };

*/
/* Chunks of Bytecode main-chunk-line < Scanning on Demand args
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8, 123) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8, 123) };
*/
/* A Virtual Machine main-chunk < Scanning on Demand args

    constant = unsafe { addConstant(&mut chunk as *mut Chunk, 3.4) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8, 123) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8, 123) };

    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_ADD as u8, 123) };

    constant = unsafe { addConstant(&mut chunk as *mut Chunk, 5.6) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_CONSTANT as u8, 123) };
    unsafe { writeChunk(&mut chunk as *mut Chunk, constant as u8, 123) };

    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_DIVIDE as u8, 123) };
*/
/* A Virtual Machine main-negate < Scanning on Demand args
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_NEGATE as u8, 123) };
*/
/* Chunks of Bytecode main-chunk < Chunks of Bytecode main-chunk-line
    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_RETURN as u8) };
*/
/* Chunks of Bytecode main-chunk-line < Scanning on Demand args

    unsafe { writeChunk(&mut chunk as *mut Chunk, OP_RETURN as u8, 123) };
*/
/* Chunks of Bytecode main-disassemble-chunk < Scanning on Demand args

    unsafe { disassembleChunk(&mut chunk as *mut Chunk, "test chunk") };
*/
/* A Virtual Machine main-interpret < Scanning on Demand args
    let _ = unsafe { interpret(&mut chunk as *mut Chunk) };
*/
//> Scanning on Demand args
    let mut args: Vec<String> = args().collect();
    if args.len() == 1 {
        repl()
    } else if args.len() == 2 {
        runFile(&args[1])
    } else {
        eprint!("Usage: lox [file]\n");
        process::exit(64);
    }

    unsafe { freeVM() };
//< Scanning on Demand args
/* A Virtual Machine main-free-vm < Scanning on Demand args
    unsafe { freeVM() };
*/
/* Chunks of Bytecode main-chunk < Scanning on Demand args
    unsafe { freeChunk(&mut chunk as *mut Chunk as *mut Chunk) };
*/
}
