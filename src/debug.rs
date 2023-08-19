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
    unsafe { printValue(unsafe { *(*chunk).constants.values.offset(constant as isize) }) };
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
