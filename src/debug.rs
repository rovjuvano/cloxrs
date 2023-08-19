//> Chunks of Bytecode debug-c
use ::core::mem::*;
use ::std::*;

//> Chunks of Bytecode debug-h
pub use crate::chunk::*;

// no need to forward declare disassembleChunk
// no need to forward declare disassembleInstruction
//< Chunks of Bytecode debug-h
//> Closures debug-include-object
use crate::object::*;
//< Closures debug-include-object
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
//> Methods and Initializers invoke-instruction
unsafe fn invokeInstruction(mut name: &str, mut chunk: *mut Chunk,
        mut offset: isize) -> isize {
    let mut constant: u8 = unsafe { *(*chunk).code.offset(offset + 1) };
    let mut argCount: u8 = unsafe { *(*chunk).code.offset(offset + 2) };
    print!("{:<16} ({} args) {:4} ", name, argCount, constant);
    unsafe { printValue(unsafe { (*(*chunk).constants.values.offset(constant as isize)).clone() }) };
    print!("'\n");
    return offset + 3;
}
//< Methods and Initializers invoke-instruction
//> simple-instruction
fn simpleInstruction(mut name: &str, mut offset: isize) -> isize {
    print!("{}\n", name);
    return offset + 1;
}
//< simple-instruction
//> Local Variables byte-instruction
unsafe fn byteInstruction(mut name: &str, mut chunk: *mut Chunk,
        mut offset: isize) -> isize {
    let mut slot: u8 = unsafe { *(*chunk).code.offset(offset + 1) };
    print!("{:<16} {:4}\n", name, slot);
    return offset + 2; // [debug]
}
//< Local Variables byte-instruction
//> Jumping Back and Forth jump-instruction
unsafe fn jumpInstruction(mut name: &str, mut sign: isize,
        mut chunk: *mut Chunk, mut offset: isize) -> isize {
    let mut jump: u16 = (unsafe { *(*chunk).code.offset(offset + 1) } as u16) << 8;
    jump |= unsafe { *(*chunk).code.offset(offset + 2) } as u16;
    print!("{:<16} {:4} -> {}\n", name, offset,
        offset + 3 + sign * jump as isize);
    return offset + 3;
}
//< Jumping Back and Forth jump-instruction
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
//> Local Variables disassemble-local
        OP_GET_LOCAL =>
            unsafe { byteInstruction("OP_GET_LOCAL", chunk, offset) },
        OP_SET_LOCAL =>
            unsafe { byteInstruction("OP_SET_LOCAL", chunk, offset) },
//< Local Variables disassemble-local
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
//> Closures disassemble-upvalue-ops
        OP_GET_UPVALUE =>
            unsafe { byteInstruction("OP_GET_UPVALUE", chunk, offset) },
        OP_SET_UPVALUE =>
            unsafe { byteInstruction("OP_SET_UPVALUE", chunk, offset) },
//< Closures disassemble-upvalue-ops
//> Classes and Instances disassemble-property-ops
        OP_GET_PROPERTY =>
            unsafe { constantInstruction("OP_GET_PROPERTY", chunk, offset) },
        OP_SET_PROPERTY =>
            unsafe { constantInstruction("OP_SET_PROPERTY", chunk, offset) },
//< Classes and Instances disassemble-property-ops
//> Superclasses disassemble-get-super
        OP_GET_SUPER =>
            unsafe { constantInstruction("OP_GET_SUPER", chunk, offset) },
//< Superclasses disassemble-get-super
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
//> Jumping Back and Forth disassemble-jump
        OP_JUMP =>
            unsafe { jumpInstruction("OP_JUMP", 1, chunk, offset) },
        OP_JUMP_IF_FALSE =>
            unsafe { jumpInstruction("OP_JUMP_IF_FALSE", 1, chunk, offset) },
//< Jumping Back and Forth disassemble-jump
//> Jumping Back and Forth disassemble-loop
        OP_LOOP =>
            unsafe { jumpInstruction("OP_LOOP", -1, chunk, offset) },
//< Jumping Back and Forth disassemble-loop
//> Calls and Functions disassemble-call
        OP_CALL =>
            unsafe { byteInstruction("OP_CALL", chunk, offset) },
//< Calls and Functions disassemble-call
//> Methods and Initializers disassemble-invoke
        OP_INVOKE =>
            unsafe { invokeInstruction("OP_INVOKE", chunk, offset) },
//< Methods and Initializers disassemble-invoke
//> Superclasses disassemble-super-invoke
        OP_SUPER_INVOKE =>
            unsafe { invokeInstruction("OP_SUPER_INVOKE", chunk, offset) },
//< Superclasses disassemble-super-invoke
//> Closures disassemble-closure
        OP_CLOSURE => {
            offset += 1;
            let mut constant: u8 = unsafe { *(*chunk).code.offset(offset) };
            offset += 1;
            print!("{:<16} {:4} ", "OP_CLOSURE", constant);
            unsafe { printValue(unsafe { (*(*chunk).constants.values.offset(constant as isize)).clone() }) };
            print!("\n");
//> disassemble-upvalues

            let mut function: *mut ObjFunction = unsafe { AS_FUNCTION!(unsafe {
                (*(*chunk).constants.values.offset(constant as isize)).clone() }) };
            for _ in 0..unsafe { (*function).upvalueCount } {
                let mut isLocal: isize = unsafe { *(*chunk).code.offset(offset) } as isize;
                offset += 1;
                let mut index: isize = unsafe { *(*chunk).code.offset(offset) } as isize;
                offset += 1;
                print!("{:04}      |                     {} {}\n",
                    offset - 2, if isLocal == 1 { "local" } else { "upvalue" }, index);
            }

//< disassemble-upvalues
            return offset;
        }
//< Closures disassemble-closure
//> Closures disassemble-close-upvalue
        OP_CLOSE_UPVALUE =>
            simpleInstruction("OP_CLOSE_UPVALUE", offset),
//< Closures disassemble-close-upvalue
        OP_RETURN =>
            simpleInstruction("OP_RETURN", offset),
//> Classes and Instances disassemble-class
        OP_CLASS =>
            unsafe { constantInstruction("OP_CLASS", chunk, offset) },
//< Classes and Instances disassemble-class
//> Superclasses disassemble-inherit
        OP_INHERIT =>
            simpleInstruction("OP_INHERIT", offset),
//< Superclasses disassemble-inherit
//> Methods and Initializers disassemble-method
        OP_METHOD =>
            unsafe { constantInstruction("OP_METHOD", chunk, offset) },
//< Methods and Initializers disassemble-method
        #[allow(unreachable_patterns)]
        _ => {
            print!("Unknown opcode {}\n", instruction as u8);
            offset + 1
        }
    };
}
//< disassemble-instruction
