//> Chunks of Bytecode debug-c
use ::core::mem::*;
use ::std::*;

//> Chunks of Bytecode debug-h
pub use crate::chunk::*;

// no need to forward declare disassembleChunk
// no need to forward declare disassembleInstruction
//< Chunks of Bytecode debug-h
//> debug-include-value
#[allow(unused_imports)]
use crate::value::*;
//< debug-include-value

/* Scanning on Demand args < Compiling Expressions dump-chunk
#[allow(dead_code)]
*/
//> Compiling Expressions dump-chunk
#[cfg_attr(not(DEBUG_PRINT_CODE), allow(dead_code))]
//< Compiling Expressions dump-chunk
pub unsafe fn disassembleChunk(mut chunk: *mut Chunk, mut name: &str) {
    print!("== {} ==\n", name);

    let mut offset: isize = 0;
    while offset < unsafe { (*chunk).count } {
        offset = unsafe { disassembleInstruction(chunk, offset) };
    }
}
//> constant-instruction
unsafe fn constantInstruction(mut name: &str, mut chunk: *mut Chunk,
        mut offset: isize) -> isize {
    let mut constant: u8 = unsafe { *(*chunk).code.offset(offset + 1) };
    print!("{:<16} {:4} '", name, constant);
/* Chunks of Bytecode constant-instruction < Types of Values value-type
    unsafe { printValue(unsafe { *(*chunk).constants.values.offset(constant as isize) }) };
*/
//> Types of Values value-type
    unsafe { printValue(unsafe { (*(*chunk).constants.values.offset(constant as isize)).clone() }) };
//< Types of Values value-type
    print!("'\n");
//> return-after-operand
    return offset + 2;
//< return-after-operand
}
//< constant-instruction
//> simple-instruction
fn simpleInstruction(mut name: &str, mut offset: isize) -> isize {
    print!("{}\n", name);
    return offset + 1;
}
//< simple-instruction
//> disassemble-instruction
pub unsafe fn disassembleInstruction(mut chunk: *mut Chunk, mut offset: isize) -> isize {
    print!("{:04} ", offset);
//> show-location
    if offset > 0 && unsafe { *(*chunk).lines.offset(offset) } ==
            unsafe { *(*chunk).lines.offset(offset - 1) } {
        print!("   | ");
    } else {
        print!("{:4} ", unsafe { *(*chunk).lines.offset(offset) });
    }
//< show-location

    let mut instruction: OpCode = unsafe { transmute(unsafe { *(*chunk).code.offset(offset) }) };
    return match instruction {
//> disassemble-constant
        OP_CONSTANT =>
            unsafe { constantInstruction("OP_CONSTANT", chunk, offset) },
//< disassemble-constant
//> Types of Values disassemble-literals
        OP_NIL =>
            simpleInstruction("OP_NIL", offset),
        OP_TRUE =>
            simpleInstruction("OP_TRUE", offset),
        OP_FALSE =>
            simpleInstruction("OP_FALSE", offset),
//< Types of Values disassemble-literals
//> Global Variables disassemble-pop
        OP_POP =>
            simpleInstruction("OP_POP", offset),
//< Global Variables disassemble-pop
//> Global Variables disassemble-get-global
        OP_GET_GLOBAL =>
            unsafe { constantInstruction("OP_GET_GLOBAL", chunk, offset) },
//< Global Variables disassemble-get-global
//> Global Variables disassemble-define-global
        OP_DEFINE_GLOBAL =>
            unsafe { constantInstruction("OP_DEFINE_GLOBAL", chunk, offset) },
//< Global Variables disassemble-define-global
//> Global Variables disassemble-set-global
        OP_SET_GLOBAL =>
            unsafe { constantInstruction("OP_SET_GLOBAL", chunk, offset) },
//< Global Variables disassemble-set-global
//> Types of Values disassemble-comparison
        OP_EQUAL =>
            simpleInstruction("OP_EQUAL", offset),
        OP_GREATER =>
            simpleInstruction("OP_GREATER", offset),
        OP_LESS =>
            simpleInstruction("OP_LESS", offset),
//< Types of Values disassemble-comparison
//> A Virtual Machine disassemble-binary
        OP_ADD =>
            simpleInstruction("OP_ADD", offset),
        OP_SUBTRACT =>
            simpleInstruction("OP_SUBTRACT", offset),
        OP_MULTIPLY =>
            simpleInstruction("OP_MULTIPLY", offset),
        OP_DIVIDE =>
            simpleInstruction("OP_DIVIDE", offset),
//> Types of Values disassemble-not
        OP_NOT =>
            simpleInstruction("OP_NOT", offset),
//< Types of Values disassemble-not
//< A Virtual Machine disassemble-binary
//> A Virtual Machine disassemble-negate
        OP_NEGATE =>
            simpleInstruction("OP_NEGATE", offset),
//< A Virtual Machine disassemble-negate
//> Global Variables disassemble-print
        OP_PRINT =>
            simpleInstruction("OP_PRINT", offset),
//< Global Variables disassemble-print
        OP_RETURN =>
            simpleInstruction("OP_RETURN", offset),
        #[allow(unreachable_patterns)]
        _ => {
            print!("Unknown opcode {}\n", instruction as u8);
            offset + 1
        }
    };
}
//< disassemble-instruction
