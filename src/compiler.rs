//> Scanning on Demand compiler-c
use ::std::*;

use crate::common::*;
//> Scanning on Demand compiler-h
// no need to forward declare compile
//< Scanning on Demand compiler-h
use crate::scanner::*;

pub unsafe fn compile(mut source: *const u8) {
    unsafe { initScanner(source) };
//> dump-tokens
    let mut line: isize = -1;
    loop {
        let mut token: Token = unsafe { scanToken() };
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        print!("{:2} '{}'\n", token.r#type.clone() as u8,
            unsafe { str_from_raw_parts!(token.start, token.length as usize) }); // [format]

        if token.r#type.clone() as u8 == TOKEN_EOF as u8 { break; }
    }
//< dump-tokens
}
