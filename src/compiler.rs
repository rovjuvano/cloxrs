//> Scanning on Demand compiler-c
//> Compiling Expressions binary
use ::core::mem::*;
//< Compiling Expressions binary
//> Compiling Expressions compiling-chunk
use ::core::ptr::*;
//< Compiling Expressions compiling-chunk
use ::std::*;
//> Compiling Expressions compiler-include-stdlib
// no need for additional includes here
//< Compiling Expressions compiler-include-stdlib

use crate::common::*;
//> Scanning on Demand compiler-h
//> Compiling Expressions compile-h
pub use crate::vm::*;

//< Compiling Expressions compile-h
/* Scanning on Demand compiler-h < Compiling Expressions compile-h
// no need to forward declare compile
*/
//> Compiling Expressions compile-h
// no need to forward declare compile
//< Compiling Expressions compile-h
//< Scanning on Demand compiler-h
use crate::scanner::*;
//> Compiling Expressions include-debug

#[cfg(DEBUG_PRINT_CODE)]
use crate::debug::*;
//< Compiling Expressions include-debug
//> Compiling Expressions compile-h
// A missing include which the compiler can in some cases resolve among the
// circular dependencies of the indiscriminate wildcard imports and exports.
// As clox doesn't have this, only including when the compiler needs it.
#[cfg(not(DEBUG_PRINT_CODE))]
use crate::chunk::*;
//< Compiling Expressions compile-h
//> Compiling Expressions parser

#[derive(Clone)] // Copy too but made explicit
struct Parser {
    pub current: Token,
    pub previous: Token,
//> had-error-field
    pub hadError: bool,
//< had-error-field
//> panic-mode-field
    pub panicMode: bool,
//< panic-mode-field
}
//> precedence

#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT,  // =
    #[allow(dead_code)]
    PREC_OR,          // or
    #[allow(dead_code)]
    PREC_AND,         // and
    #[allow(dead_code)]
    PREC_EQUALITY,    // == !=
    #[allow(dead_code)]
    PREC_COMPARISON,  // < > <= >=
    PREC_TERM,        // + -
    PREC_FACTOR,      // * /
    PREC_UNARY,       // ! -
    #[allow(dead_code)]
    PREC_CALL,        // . ()
    #[allow(dead_code)]
    PREC_PRIMARY,
}
use Precedence::*;
//< precedence
//> parse-fn-type

type ParseFn = unsafe fn() -> ();
//< parse-fn-type
//> parse-rule

#[derive(Clone)] // Copy too but made explicit
struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}
//< parse-rule

static mut parser: Parser = unsafe { uninit_static!(Parser) };
//< Compiling Expressions parser
//> Compiling Expressions compiling-chunk
static mut compilingChunk: *mut Chunk = null_mut();

unsafe fn currentChunk() -> *mut Chunk {
    return unsafe { compilingChunk };
}

//< Compiling Expressions compiling-chunk
//> Compiling Expressions error-at
unsafe fn errorAt(mut token: *mut Token, mut message: &str) {
//> check-panic-mode
    if unsafe { parser.panicMode } { return; }
//< check-panic-mode
//> set-panic-mode
    unsafe { parser.panicMode = true };
//< set-panic-mode
    eprint!("[line {}] Error", unsafe { (*token).line });

    if unsafe { (*token).r#type.clone() } as u8 == TOKEN_EOF as u8 {
        eprint!(" at end");
    } else if unsafe { (*token).r#type.clone() } as u8 == TOKEN_ERROR as u8 {
        // Nothing.
    } else {
        eprint!(" at '{}'", unsafe { str_from_raw_parts!(unsafe { (*token).start }, unsafe { (*token).length }) });
    }

    eprint!(": {}\n", message);
    unsafe { parser.hadError = true };
}
//< Compiling Expressions error-at
//> Compiling Expressions error
unsafe fn error(mut message: &str) {
    unsafe { errorAt(unsafe { &mut parser.previous } as *mut Token, message) };
}
//< Compiling Expressions error
//> Compiling Expressions error-at-current
unsafe fn errorAtCurrent(mut message: &str) {
    unsafe { errorAt(unsafe { &mut parser.current } as *mut Token, message) };
}
//< Compiling Expressions error-at-current
//> Compiling Expressions advance

unsafe fn advance() {
    unsafe { parser.previous = unsafe { parser.current.clone() } };

    loop {
        unsafe { parser.current = unsafe { scanToken() } };
        if unsafe { parser.current.r#type.clone() } as u8 != TOKEN_ERROR as u8 { break; }

        unsafe { errorAtCurrent(unsafe { str_from_raw_parts!(unsafe { parser.current.start }, unsafe { parser.current.length }) }) };
    }
}
//< Compiling Expressions advance
//> Compiling Expressions consume
unsafe fn consume(mut r#type: TokenType, mut message: &str) {
    if unsafe { parser.current.r#type.clone() } as u8 == r#type as u8 {
        unsafe { advance() };
        return;
    }

    unsafe { errorAtCurrent(message) };
}
//< Compiling Expressions consume
//> Compiling Expressions emit-byte
unsafe fn emitByte(mut byte: u8) {
    unsafe { writeChunk(unsafe { currentChunk() }, byte, unsafe { parser.previous.line }) };
}
//< Compiling Expressions emit-byte
//> Compiling Expressions emit-bytes
unsafe fn emitBytes(mut byte1: u8, mut byte2: u8) {
    unsafe { emitByte(byte1) };
    unsafe { emitByte(byte2) };
}
//< Compiling Expressions emit-bytes
//> Compiling Expressions emit-return
unsafe fn emitReturn() {
    unsafe { emitByte(OP_RETURN as u8) };
}
//< Compiling Expressions emit-return
//> Compiling Expressions make-constant
unsafe fn makeConstant(mut value: Value) -> u8 {
    let mut constant: isize = unsafe { addConstant(unsafe { currentChunk() }, value) };
    if constant > u8::MAX as isize {
        unsafe { error("Too many constants in one chunk.") };
        return 0;
    }

    return constant as u8;
}
//< Compiling Expressions make-constant
//> Compiling Expressions emit-constant
unsafe fn emitConstant(mut value: Value) {
    unsafe { emitBytes(OP_CONSTANT as u8, unsafe { makeConstant(value) }) };
}
//< Compiling Expressions emit-constant
//> Compiling Expressions end-compiler
unsafe fn endCompiler() {
    unsafe { emitReturn() };
//> dump-chunk
    #[cfg(DEBUG_PRINT_CODE)]
    if !unsafe { parser.hadError } {
        unsafe { disassembleChunk(unsafe { currentChunk() }, "code") };
    }
//< dump-chunk
}
//< Compiling Expressions end-compiler
//> Compiling Expressions forward-declarations

// no need to forward declare expression
// no need to forward declare getRule
// no need to forward declare parsePrecedence

//< Compiling Expressions forward-declarations
//> Compiling Expressions binary
unsafe fn binary() {
    let mut operatorType: TokenType = unsafe { parser.previous.r#type.clone() };
    let mut rule: *mut ParseRule = getRule(operatorType.clone());
    unsafe { parsePrecedence(unsafe { transmute::<u8, Precedence>(unsafe { (*rule).precedence.clone() } as u8 + 1) }) };

    match operatorType {
        TOKEN_PLUS          => unsafe { emitByte(OP_ADD as u8) },
        TOKEN_MINUS         => unsafe { emitByte(OP_SUBTRACT as u8) },
        TOKEN_STAR          => unsafe { emitByte(OP_MULTIPLY as u8) },
        TOKEN_SLASH         => unsafe { emitByte(OP_DIVIDE as u8) },
        _ => {} // Unreachable.
    }
}
//< Compiling Expressions binary
//> Compiling Expressions grouping
unsafe fn grouping() {
    unsafe { expression() };
    unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression.") };
}
//< Compiling Expressions grouping
//> Compiling Expressions number
unsafe fn number() {
    let mut value: f64 = unsafe { str_from_raw_parts!(unsafe { parser.previous.start }, unsafe { parser.previous.length }) }.parse::<f64>().unwrap();
    unsafe { emitConstant(value) };
}
//< Compiling Expressions number
//> Compiling Expressions unary
unsafe fn unary() {
    let mut operatorType: TokenType = unsafe { parser.previous.r#type.clone() };

    // Compile the operand.
/* Compiling Expressions unary < Compiling Expressions unary-operand
    unsafe { expression() };
*/
//> unary-operand
    unsafe { parsePrecedence(PREC_UNARY) };
//< unary-operand

    // Emit the operator instruction.
    match operatorType {
        TOKEN_MINUS => unsafe { emitByte(OP_NEGATE as u8) },
        _ => {} // Unreachable.
    }
}
//< Compiling Expressions unary
//> Compiling Expressions rules
const COUNT_OF_TOKEN_TYPES: usize = 40; // ::std::mem::variant_count::<TokenType>();
type ParseRules = [ParseRule; COUNT_OF_TOKEN_TYPES];
fn null_parse_rule() {}
macro_rules! parse_rules {
    (@fn NULL) => { null_parse_rule };
    (@fn $fn:expr) => { $fn };
    ($([$token_type:ident] = {$prefix:tt, $infix:tt, $precedence:ident},)*) => {{
        let mut xs: ParseRules = unsafe { uninit::<ParseRules>() };
        $(
            xs[$token_type as usize] = ParseRule {
                prefix: parse_rules!(@fn $prefix),
                infix: parse_rules!(@fn $infix),
                precedence: $precedence,
            };
        )*
        xs
    }};
}
static mut rules: ParseRules = parse_rules!{
    [TOKEN_LEFT_PAREN]    = {grouping, NULL,   PREC_NONE},
    [TOKEN_RIGHT_PAREN]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LEFT_BRACE]    = {NULL,     NULL,   PREC_NONE}, // [big]
    [TOKEN_RIGHT_BRACE]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_COMMA]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_DOT]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_MINUS]         = {unary,    binary, PREC_TERM},
    [TOKEN_PLUS]          = {NULL,     binary, PREC_TERM},
    [TOKEN_SEMICOLON]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_SLASH]         = {NULL,     binary, PREC_FACTOR},
    [TOKEN_STAR]          = {NULL,     binary, PREC_FACTOR},
    [TOKEN_BANG]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_BANG_EQUAL]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_EQUAL_EQUAL]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_GREATER]       = {NULL,     NULL,   PREC_NONE},
    [TOKEN_GREATER_EQUAL] = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LESS]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LESS_EQUAL]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_STRING]        = {NULL,     NULL,   PREC_NONE},
    [TOKEN_NUMBER]        = {number,   NULL,   PREC_NONE},
    [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FALSE]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FUN]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
    [TOKEN_NIL]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
    [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
    [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_TRUE]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_VAR]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_WHILE]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_ERROR]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_EOF]           = {NULL,     NULL,   PREC_NONE},
};
//< Compiling Expressions rules
//> Compiling Expressions parse-precedence
unsafe fn parsePrecedence(mut precedence: Precedence) {
/* Compiling Expressions parse-precedence < Compiling Expressions precedence-body
    // What goes here?
*/
//> precedence-body
    unsafe { advance() };
    let mut prefixRule: ParseFn = unsafe { (*getRule(unsafe { parser.previous.r#type.clone() })).prefix };
    if prefixRule == null_parse_rule {
        unsafe { error("Expect expression.") };
        return;
    }

    unsafe { prefixRule() };
//> infix

    while precedence.clone() as u8 <= unsafe { (*getRule(unsafe { parser.current.r#type.clone() })).precedence.clone() as u8 } {
        unsafe { advance() };
        let mut infixRule: ParseFn = unsafe { (*getRule(unsafe { parser.previous.r#type.clone() })).infix };
        unsafe { infixRule() };
    }
//< infix
//< precedence-body
}
//< Compiling Expressions parse-precedence
//> Compiling Expressions get-rule
fn getRule(mut r#type: TokenType) -> *mut ParseRule {
    return unsafe { &mut rules[r#type as usize] } as *mut ParseRule;
}
//< Compiling Expressions get-rule
//> Compiling Expressions expression
unsafe fn expression() {
/* Compiling Expressions expression < Compiling Expressions expression-body
    // What goes here?
*/
//> expression-body
    unsafe { parsePrecedence(PREC_ASSIGNMENT) };
//< expression-body
}
//< Compiling Expressions expression

/* Scanning on Demand compiler-c < Compiling Expressions compile-signature
pub unsafe fn compile(mut source: *const u8) {
*/
//> Compiling Expressions compile-signature
pub unsafe fn compile(mut source: *const u8, mut chunk: *mut Chunk) -> bool {
//< Compiling Expressions compile-signature
    unsafe { initScanner(source) };
/* Scanning on Demand dump-tokens < Compiling Expressions compile-chunk
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
*/
//> Compiling Expressions init-compile-chunk
    unsafe { compilingChunk = chunk };
//< Compiling Expressions init-compile-chunk
//> Compiling Expressions compile-chunk
//> init-parser-error

    unsafe { parser.hadError = false };
    unsafe { parser.panicMode = false };

//< init-parser-error
    unsafe { advance() };
    unsafe { expression() };
    unsafe { consume(TOKEN_EOF, "Expect end of expression.") };
//< Compiling Expressions compile-chunk
//> Compiling Expressions finish-compile
    unsafe { endCompiler() };
//< Compiling Expressions finish-compile
//> Compiling Expressions return-had-error
    return !unsafe { parser.hadError };
//< Compiling Expressions return-had-error
}
