//> Scanning on Demand compiler-c
//> Local Variables declare-variable
use ::core::iter::*;
//< Local Variables declare-variable
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
//> Local Variables compiler-include-string
// no need for additional includes here
//< Local Variables compiler-include-string

use crate::common::*;
//> Scanning on Demand compiler-h
//> Strings compiler-include-object
pub use crate::object::*;
//< Strings compiler-include-object
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
/* Compiling Expressions precedence < Jumping Back and Forth table-or
    #[allow(dead_code)]
*/
    PREC_OR,          // or
/* Compiling Expressions precedence < Jumping Back and Forth table-and
    #[allow(dead_code)]
*/
    PREC_AND,         // and
/* Compiling Expressions precedence < Types of Values table-equal
    #[allow(dead_code)]
*/
    PREC_EQUALITY,    // == !=
/* Compiling Expressions precedence < Types of Values table-comparisons
    #[allow(dead_code)]
*/
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

//< parse-fn-type
/* Compiling Expressions parse-fn-type < Global Variables parse-fn-type
type ParseFn = unsafe fn() -> ();
*/
//> Global Variables parse-fn-type
type ParseFn = unsafe fn(canAssign: bool) -> ();
//< Global Variables parse-fn-type
//> parse-rule

#[derive(Clone)] // Copy too but made explicit
struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}
//< parse-rule
//> Local Variables local-struct

#[derive(Clone)] // Copy too but made explicit
struct Local {
    pub name: Token,
    pub depth: isize,
}
//< Local Variables local-struct
//> Local Variables compiler-struct

#[derive(Clone)] // Copy too but made explicit
struct Compiler {
    pub locals: [Local; UINT8_COUNT as usize],
    pub localCount: isize,
    pub scopeDepth: isize,
}
//< Local Variables compiler-struct

static mut parser: Parser = unsafe { uninit_static!(Parser) };
//< Compiling Expressions parser
//> Local Variables current-compiler
static mut current: *mut Compiler = null_mut();
//< Local Variables current-compiler
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
//> Global Variables check
unsafe fn check(mut r#type: TokenType) -> bool {
    return unsafe { parser.current.r#type.clone() } as u8 == r#type as u8;
}
//< Global Variables check
//> Global Variables match
unsafe fn r#match(mut r#type: TokenType) -> bool {
    if !unsafe { check(r#type) } { return false; }
    unsafe { advance() };
    return true;
}
//< Global Variables match
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
//> Jumping Back and Forth emit-loop
unsafe fn emitLoop(mut loopStart: isize) {
    unsafe { emitByte(OP_LOOP as u8) };

    let mut offset: isize = unsafe { (*unsafe { currentChunk() }).count } - loopStart + 2;
    if offset > u16::MAX as isize { unsafe { error("Loop body too large.") }; }

    unsafe { emitByte(((offset >> 8) & 0xff) as u8) };
    unsafe { emitByte((offset & 0xff) as u8) };
}
//< Jumping Back and Forth emit-loop
//> Jumping Back and Forth emit-jump
unsafe fn emitJump(mut instruction: OpCode) -> isize {
    unsafe { emitByte(instruction as u8) };
    unsafe { emitByte(0xff) };
    unsafe { emitByte(0xff) };
    return unsafe { (*unsafe { currentChunk() }).count } - 2;
}
//< Jumping Back and Forth emit-jump
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
//> Jumping Back and Forth patch-jump
unsafe fn patchJump(mut offset: isize) {
    // -2 to adjust for the bytecode for the jump offset itself.
    let mut jump: isize = unsafe { (*unsafe { currentChunk() }).count } - offset - 2;

    if jump > u16::MAX as isize {
        unsafe { error("Too much code to jump over.") };
    }

    unsafe { *(*unsafe { currentChunk() }).code.offset(offset) = ((jump >> 8) & 0xff) as u8 };
    unsafe { *(*unsafe { currentChunk() }).code.offset(offset + 1) = (jump & 0xff) as u8 };
}
//< Jumping Back and Forth patch-jump
//> Local Variables init-compiler
unsafe fn initCompiler(mut compiler: *mut Compiler) {
    unsafe { (*compiler).localCount = 0 };
    unsafe { (*compiler).scopeDepth = 0 };
    unsafe { current = compiler };
}
//< Local Variables init-compiler
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
//> Local Variables begin-scope
unsafe fn beginScope() {
    unsafe { (*current).scopeDepth += 1 };
}
//< Local Variables begin-scope
//> Local Variables end-scope
unsafe fn endScope() {
    unsafe { (*current).scopeDepth -= 1 };
//> pop-locals

    while unsafe { (*current).localCount } > 0 &&
        unsafe { (*current).locals[unsafe { (*current).localCount } as usize - 1].depth } >
            unsafe { (*current).scopeDepth } {
        unsafe { emitByte(OP_POP as u8) };
        unsafe { (*current).localCount -= 1 };
    }
//< pop-locals
}
//< Local Variables end-scope
//> Compiling Expressions forward-declarations

// no need to forward declare expression
//> Global Variables forward-declarations
// no need to forward declare statement
// no need to forward declare declaration
//< Global Variables forward-declarations
// no need to forward declare getRule
// no need to forward declare parsePrecedence

//< Compiling Expressions forward-declarations
//> Global Variables identifier-constant
unsafe fn identifierConstant(mut name: *mut Token) -> u8 {
    return unsafe { makeConstant(OBJ_VAL!(unsafe { copyString(
        unsafe { (*name).start }, unsafe { (*name).length }) })) };
}
//< Global Variables identifier-constant
//> Local Variables identifiers-equal
unsafe fn identifiersEqual(mut a: *mut Token, mut b: *mut Token) -> bool {
    if unsafe { (*a).length } != unsafe { (*b).length } { return false; }
    return unsafe { memcmp(unsafe { (*a).start }, unsafe { (*b).start }, unsafe { (*a).length } as usize) } == 0;
}
//< Local Variables identifiers-equal
//> Local Variables resolve-local
unsafe fn resolveLocal(mut compiler: *mut Compiler, mut name: *mut Token) -> isize {
    for mut i in (0..unsafe { (*compiler).localCount }).rev() {
        let mut local: *mut Local = unsafe { &mut (*compiler).locals[i as usize] } as *mut Local;
        if unsafe { identifiersEqual(name, unsafe { &mut (*local).name } as *mut Token) } {
//> own-initializer-error
            if unsafe { (*local).depth } == -1 {
                unsafe { error("Can't read local variable in its own initializer.") };
            }
//< own-initializer-error
            return i as isize;
        }
    }

    return -1;
}
//< Local Variables resolve-local
//> Local Variables add-local
unsafe fn addLocal(mut name: Token) {
//> too-many-locals
    if unsafe { (*current).localCount } == UINT8_COUNT {
        unsafe { error("Too many local variables in function.") };
        return;
    }

//< too-many-locals
    let mut local: *mut Local = unsafe { &mut (*current).locals[unsafe { (*current).localCount } as usize] } as *mut Local;
    unsafe { (*current).localCount += 1 };
    unsafe { (*local).name = name.clone() };
/* Local Variables add-local < Local Variables declare-undefined
    unsafe { (*local).depth = unsafe { (*current).scopeDepth } };
*/
//> declare-undefined
    unsafe { (*local).depth = -1 };
//< declare-undefined
}
//< Local Variables add-local
//> Local Variables declare-variable
unsafe fn declareVariable() {
    if unsafe { (*current).scopeDepth } == 0 { return; }

    let mut name: *mut Token = unsafe { &mut parser.previous } as *mut Token;
//> existing-in-scope
    for mut i in (0..unsafe { (*current).localCount }).rev() {
        let mut local: *mut Local = unsafe { &mut (*current).locals[i as usize] } as *mut Local;
        if unsafe { (*local).depth } != -1 && unsafe { (*local).depth } < unsafe { (*current).scopeDepth } {
            break; // [negative]
        }

        if unsafe { identifiersEqual(name, unsafe { &mut (*local).name } as *mut Token) } {
            unsafe { error("Already a variable with this name in this scope.") };
        }
    }

//< existing-in-scope
    unsafe { addLocal(unsafe { (*name).clone() }) };
}
//< Local Variables declare-variable
//> Global Variables parse-variable
unsafe fn parseVariable(mut errorMessage: &str) -> u8 {
    unsafe { consume(TOKEN_IDENTIFIER, errorMessage) };
//> Local Variables parse-local

    unsafe { declareVariable() };
    if unsafe { (*current).scopeDepth } > 0 { return 0; }

//< Local Variables parse-local
    return unsafe { identifierConstant(unsafe { &mut parser.previous } as *mut Token) };
}
//< Global Variables parse-variable
//> Local Variables mark-initialized
unsafe fn markInitialized() {
    unsafe { (*current).locals[unsafe { (*current).localCount } as usize - 1].depth =
        unsafe { (*current).scopeDepth } };
}
//< Local Variables mark-initialized
//> Global Variables define-variable
unsafe fn defineVariable(mut global: u8) {
//> Local Variables define-variable
    if unsafe { (*current).scopeDepth > 0 } {
//> define-local
        unsafe { markInitialized() };
//< define-local
        return;
    }

//< Local Variables define-variable
    unsafe { emitBytes(OP_DEFINE_GLOBAL as u8, global) };
}
//< Global Variables define-variable
//> Jumping Back and Forth and
unsafe fn and(mut _canAssign: bool) {
    let mut endJump: isize = unsafe { emitJump(OP_JUMP_IF_FALSE) };

    unsafe { emitByte(OP_POP as u8) };
    unsafe { parsePrecedence(PREC_AND) };

    unsafe { patchJump(endJump) };
}
//< Jumping Back and Forth and
//> Compiling Expressions binary
/* Compiling Expressions binary < Global Variables binary
unsafe fn binary() {
*/
//> Global Variables binary
unsafe fn binary(mut _canAssign: bool) {
//< Global Variables binary
    let mut operatorType: TokenType = unsafe { parser.previous.r#type.clone() };
    let mut rule: *mut ParseRule = getRule(operatorType.clone());
    unsafe { parsePrecedence(unsafe { transmute::<u8, Precedence>(unsafe { (*rule).precedence.clone() } as u8 + 1) }) };

    match operatorType {
//> Types of Values comparison-operators
        TOKEN_BANG_EQUAL    => unsafe { emitBytes(OP_EQUAL as u8, OP_NOT as u8) },
        TOKEN_EQUAL_EQUAL   => unsafe { emitByte(OP_EQUAL as u8) },
        TOKEN_GREATER       => unsafe { emitByte(OP_GREATER as u8) },
        TOKEN_GREATER_EQUAL => unsafe { emitBytes(OP_LESS as u8, OP_NOT as u8) },
        TOKEN_LESS          => unsafe { emitByte(OP_LESS as u8) },
        TOKEN_LESS_EQUAL    => unsafe { emitBytes(OP_GREATER as u8, OP_NOT as u8) },
//< Types of Values comparison-operators
        TOKEN_PLUS          => unsafe { emitByte(OP_ADD as u8) },
        TOKEN_MINUS         => unsafe { emitByte(OP_SUBTRACT as u8) },
        TOKEN_STAR          => unsafe { emitByte(OP_MULTIPLY as u8) },
        TOKEN_SLASH         => unsafe { emitByte(OP_DIVIDE as u8) },
        _ => {} // Unreachable.
    }
}
//< Compiling Expressions binary
//> Types of Values parse-literal
/* Types of Values parse-literal < Global Variables parse-literal
unsafe fn literal() {
*/
//> Global Variables parse-literal
unsafe fn literal(mut _canAssign: bool) {
//< Global Variables parse-literal
    match unsafe { parser.previous.r#type.clone() } {
        TOKEN_FALSE => unsafe { emitByte(OP_FALSE as u8) },
        TOKEN_NIL => unsafe { emitByte(OP_NIL as u8) },
        TOKEN_TRUE => unsafe { emitByte(OP_TRUE as u8) },
        _ => {} // Unreachable.
    }
}
//< Types of Values parse-literal
//> Compiling Expressions grouping
/* Compiling Expressions grouping < Global Variables grouping
unsafe fn grouping() {
*/
//> Global Variables grouping
unsafe fn grouping(mut _canAssign: bool) {
//< Global Variables grouping
    unsafe { expression() };
    unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression.") };
}
//< Compiling Expressions grouping
/* Compiling Expressions number < Global Variables number
unsafe fn number() {
*/
//> Compiling Expressions number
//> Global Variables number
unsafe fn number(mut _canAssign: bool) {
//< Global Variables number
    let mut value: f64 = unsafe { str_from_raw_parts!(unsafe { parser.previous.start }, unsafe { parser.previous.length }) }.parse::<f64>().unwrap();
/* Compiling Expressions number < Types of Values const-number-val
    unsafe { emitConstant(value) };
*/
//> Types of Values const-number-val
    unsafe { emitConstant(NUMBER_VAL!(value)) };
//< Types of Values const-number-val
}
//< Compiling Expressions number
//> Jumping Back and Forth or
unsafe fn or(mut _canAssign: bool) {
    let mut elseJump: isize = unsafe { emitJump(OP_JUMP_IF_FALSE) };
    let mut endJump: isize = unsafe { emitJump(OP_JUMP) };

    unsafe { patchJump(elseJump) };
    unsafe { emitByte(OP_POP as u8) };

    unsafe { parsePrecedence(PREC_OR) };
    unsafe { patchJump(endJump) };
}
//< Jumping Back and Forth or
/* Strings parse-string < Global Variables string
unsafe fn string() {
*/
//> Strings parse-string
//> Global Variables string
unsafe fn string(mut _canAssign: bool) {
//< Global Variables string
    unsafe { emitConstant(OBJ_VAL!(unsafe { copyString(unsafe { parser.previous.start.offset(1) },
        unsafe { parser.previous.length - 2 }) })) };
}
//< Strings parse-string
/* Global Variables read-named-variable < Global Variables named-variable-signature
unsafe fn namedVariable(mut name: Token) {
*/
//> Global Variables named-variable-signature
unsafe fn namedVariable(mut name: Token, mut canAssign: bool) {
//< Global Variables named-variable-signature
/* Global Variables read-named-variable < Local Variables named-local
    let mut arg: u8 = unsafe { identifierConstant(&mut name as *mut Token) };
*/
//> Global Variables read-named-variable
//> Local Variables named-local
    #[allow(unused_assignments)]
    let (mut getOp, mut setOp): (OpCode, OpCode) = unsafe { uninit::<(OpCode, OpCode)>() };
    let mut arg: isize = unsafe { resolveLocal(unsafe { current }, &mut name as *mut Token) };
    if arg != -1 {
        getOp = OP_GET_LOCAL;
        setOp = OP_SET_LOCAL;
    } else {
        arg = unsafe { identifierConstant(&mut name as *mut Token) } as isize;
        getOp = OP_GET_GLOBAL;
        setOp = OP_SET_GLOBAL;
    }
//< Local Variables named-local
/* Global Variables read-named-variable < Global Variables named-variable
    unsafe { emitBytes(OP_GET_GLOBAL as u8, arg) };
*/
//> named-variable

/* Global Variables named-variable < Global Variables named-variable-can-assign
    if unsafe { r#match(TOKEN_EQUAL) } {
*/
//> named-variable-can-assign
    if canAssign && unsafe { r#match(TOKEN_EQUAL) } {
//< named-variable-can-assign
        unsafe { expression() };
/* Global Variables named-variable < Local Variables emit-set
        unsafe { emitBytes(OP_SET_GLOBAL as u8, arg) };
*/
//> Local Variables emit-set
        unsafe { emitBytes(setOp as u8, arg as u8) };
//< Local Variables emit-set
    } else {
/* Global Variables named-variable < Local Variables emit-get
        unsafe { emitBytes(OP_GET_GLOBAL as u8, arg) };
*/
//> Local Variables emit-get
        unsafe { emitBytes(getOp as u8, arg as u8) };
//< Local Variables emit-get
    }
//< named-variable
}
//< Global Variables read-named-variable
/* Global Variables variable-without-assign < Global Variables variable
unsafe fn variable() {
    unsafe { namedVariable(unsafe { parser.previous.clone() }) };
}
*/
//> Global Variables variable
unsafe fn variable(mut canAssign: bool) {
    unsafe { namedVariable(unsafe { parser.previous.clone() }, canAssign) };
}
//< Global Variables variable
//> Compiling Expressions unary
/* Compiling Expressions unary < Global Variables unary
unsafe fn unary() {
*/
//> Global Variables unary
unsafe fn unary(mut _canAssign: bool) {
//< Global Variables unary
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
//> Types of Values compile-not
        TOKEN_BANG => unsafe { emitByte(OP_NOT as u8) },
//< Types of Values compile-not
        TOKEN_MINUS => unsafe { emitByte(OP_NEGATE as u8) },
        _ => {} // Unreachable.
    }
}
//< Compiling Expressions unary
//> Compiling Expressions rules
const COUNT_OF_TOKEN_TYPES: usize = 40; // ::std::mem::variant_count::<TokenType>();
type ParseRules = [ParseRule; COUNT_OF_TOKEN_TYPES];
/* Compiling Expressions rules < Global Variables parse-fn-type
fn null_parse_rule() {}
*/
//> Global Variables parse-fn-type
fn null_parse_rule(mut _canAssign: bool) {}
//< Global Variables parse-fn-type
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
/* Compiling Expressions rules < Types of Values table-not
    [TOKEN_BANG]          = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-not
    [TOKEN_BANG]          = {unary,    NULL,   PREC_NONE},
//< Types of Values table-not
/* Compiling Expressions rules < Types of Values table-equal
    [TOKEN_BANG_EQUAL]    = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-equal
    [TOKEN_BANG_EQUAL]    = {NULL,     binary, PREC_EQUALITY},
//< Types of Values table-equal
    [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-comparisons
    [TOKEN_EQUAL_EQUAL]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_GREATER]       = {NULL,     NULL,   PREC_NONE},
    [TOKEN_GREATER_EQUAL] = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LESS]          = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LESS_EQUAL]    = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-comparisons
    [TOKEN_EQUAL_EQUAL]   = {NULL,     binary, PREC_EQUALITY},
    [TOKEN_GREATER]       = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_GREATER_EQUAL] = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_LESS]          = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_LESS_EQUAL]    = {NULL,     binary, PREC_COMPARISON},
//< Types of Values table-comparisons
/* Compiling Expressions rules < Global Variables table-identifier
    [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
*/
//> Global Variables table-identifier
    [TOKEN_IDENTIFIER]    = {variable, NULL,   PREC_NONE},
//< Global Variables table-identifier
/* Compiling Expressions rules < Strings table-string
    [TOKEN_STRING]        = {NULL,     NULL,   PREC_NONE},
*/
//> Strings table-string
    [TOKEN_STRING]        = {string,   NULL,   PREC_NONE},
//< Strings table-string
    [TOKEN_NUMBER]        = {number,   NULL,   PREC_NONE},
/* Compiling Expressions rules < Jumping Back and Forth table-and
    [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
*/
//> Jumping Back and Forth table-and
    [TOKEN_AND]           = {NULL,     and,    PREC_AND},
//< Jumping Back and Forth table-and
    [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-false
    [TOKEN_FALSE]         = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-false
    [TOKEN_FALSE]         = {literal,  NULL,   PREC_NONE},
//< Types of Values table-false
    [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FUN]           = {NULL,     NULL,   PREC_NONE},
    [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-nil
    [TOKEN_NIL]           = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-nil
    [TOKEN_NIL]           = {literal,  NULL,   PREC_NONE},
//< Types of Values table-nil
/* Compiling Expressions rules < Jumping Back and Forth table-or
    [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
*/
//> Jumping Back and Forth table-or
    [TOKEN_OR]            = {NULL,     or,     PREC_OR},
//< Jumping Back and Forth table-or
    [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
    [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
    [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
/* Compiling Expressions rules < Types of Values table-true
    [TOKEN_TRUE]          = {NULL,     NULL,   PREC_NONE},
*/
//> Types of Values table-true
    [TOKEN_TRUE]          = {literal,  NULL,   PREC_NONE},
//< Types of Values table-true
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

/* Compiling Expressions precedence-body < Global Variables prefix-rule
    unsafe { prefixRule() };
*/
//> Global Variables prefix-rule
    let mut canAssign: bool = precedence.clone() as u8 <= PREC_ASSIGNMENT as u8;
    unsafe { prefixRule(canAssign) };
//< Global Variables prefix-rule
//> infix

    while precedence.clone() as u8 <= unsafe { (*getRule(unsafe { parser.current.r#type.clone() })).precedence.clone() as u8 } {
        unsafe { advance() };
        let mut infixRule: ParseFn = unsafe { (*getRule(unsafe { parser.previous.r#type.clone() })).infix };
/* Compiling Expressions infix < Global Variables infix-rule
        unsafe { infixRule() };
*/
//> Global Variables infix-rule
        unsafe { infixRule(canAssign) };
//< Global Variables infix-rule
    }
//> Global Variables invalid-assign

    if canAssign && unsafe { r#match(TOKEN_EQUAL) } {
        unsafe { error("Invalid assignment target.") };
    }
//< Global Variables invalid-assign
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
//> Local Variables block
unsafe fn block() {
    while !unsafe { check(TOKEN_RIGHT_BRACE) } && !unsafe { check(TOKEN_EOF) } {
        unsafe { declaration() };
    }

    unsafe { consume(TOKEN_RIGHT_BRACE, "Expect '}' after block.") };
}
//< Local Variables block
//> Global Variables var-declaration
unsafe fn varDeclaration() {
    let mut global: u8 = unsafe { parseVariable("Expect variable name.") };

    if unsafe { r#match(TOKEN_EQUAL) } {
        unsafe { expression() };
    } else {
        unsafe { emitByte(OP_NIL as u8) };
    }
    unsafe { consume(TOKEN_SEMICOLON,
        "Expect ';' after variable declaration.") };

    unsafe { defineVariable(global) };
}
//< Global Variables var-declaration
//> Global Variables expression-statement
unsafe fn expressionStatement() {
    unsafe { expression() };
    unsafe { consume(TOKEN_SEMICOLON, "Expect ';' after expression.") };
    unsafe { emitByte(OP_POP as u8) };
}
//< Global Variables expression-statement
//> Jumping Back and Forth for-statement
unsafe fn forStatement() {
//> for-begin-scope
    unsafe { beginScope() };
//< for-begin-scope
    unsafe { consume(TOKEN_LEFT_PAREN, "Expect '(' after 'for'.") };
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-initializer
    unsafe { consume(TOKEN_SEMICOLON, "Expect ';'.") };
*/
//> for-initializer
    if unsafe { r#match(TOKEN_SEMICOLON) } {
        // No initializer.
    } else if unsafe { r#match(TOKEN_VAR) } {
        unsafe { varDeclaration() };
    } else {
        unsafe { expressionStatement() };
    }
//< for-initializer

    let mut loopStart: isize = unsafe { (*unsafe { currentChunk() }).count };
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-exit
    unsafe { consume(TOKEN_SEMICOLON, "Expect ';'.") };
*/
//> for-exit
    let mut exitJump: isize = -1;
    if !unsafe { r#match(TOKEN_SEMICOLON) } {
        unsafe { expression() };
        unsafe { consume(TOKEN_SEMICOLON, "Expect ';' after loop condition.") };

        // Jump out of the loop if the condition is false.
        exitJump = unsafe { emitJump(OP_JUMP_IF_FALSE) };
        unsafe { emitByte(OP_POP as u8) }; // Condition.
    }

//< for-exit
/* Jumping Back and Forth for-statement < Jumping Back and Forth for-increment
    unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after for clauses.") };
*/
//> for-increment
    if !unsafe { r#match(TOKEN_RIGHT_PAREN) } {
        let mut bodyJump: isize = unsafe { emitJump(OP_JUMP) };
        let mut incrementStart: isize = unsafe { (*unsafe { currentChunk() }).count };
        unsafe { expression() };
        unsafe { emitByte(OP_POP as u8) };
        unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after for clauses.") };

        unsafe { emitLoop(loopStart) };
        loopStart = incrementStart;
        unsafe { patchJump(bodyJump) };
    }
//< for-increment

    unsafe { statement() };
    unsafe { emitLoop(loopStart) };
//> exit-jump

    if exitJump != -1 {
        unsafe { patchJump(exitJump) };
        unsafe { emitByte(OP_POP as u8) }; // Condition.
    }

//< exit-jump
//> for-end-scope
    unsafe { endScope() };
//< for-end-scope
}
//< Jumping Back and Forth for-statement
//> Jumping Back and Forth if-statement
unsafe fn ifStatement() {
    unsafe { consume(TOKEN_LEFT_PAREN, "Expect '(' after 'if'.") };
    unsafe { expression() };
    unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after condition.") }; // [paren]

    let mut thenJump: isize = unsafe { emitJump(OP_JUMP_IF_FALSE) };
//> pop-then
    unsafe { emitByte(OP_POP as u8) };
//< pop-then
    unsafe { statement() };

//> jump-over-else
    let mut elseJump: isize = unsafe { emitJump(OP_JUMP) };

//< jump-over-else
    unsafe { patchJump(thenJump) };
//> pop-end
    unsafe { emitByte(OP_POP as u8) };
//< pop-end
//> compile-else

    if unsafe { r#match(TOKEN_ELSE) } { unsafe { statement() }; }
//< compile-else
//> patch-else
    unsafe { patchJump(elseJump) };
//< patch-else
}
//< Jumping Back and Forth if-statement
//> Global Variables print-statement
unsafe fn printStatement() {
    unsafe { expression() };
    unsafe { consume(TOKEN_SEMICOLON, "Expect ';' after value.") };
    unsafe { emitByte(OP_PRINT as u8) };
}
//< Global Variables print-statement
//> Jumping Back and Forth while-statement
unsafe fn whileStatement() {
//> loop-start
    let mut loopStart: isize = unsafe { (*unsafe { currentChunk() }).count };
//< loop-start
    unsafe { consume(TOKEN_LEFT_PAREN, "Expect '(' after 'while'.") };
    unsafe { expression() };
    unsafe { consume(TOKEN_RIGHT_PAREN, "Expect ')' after condition.") };

    let mut exitJump: isize = unsafe { emitJump(OP_JUMP_IF_FALSE) };
    unsafe { emitByte(OP_POP as u8) };
    unsafe { statement() };
//> loop
    unsafe { emitLoop(loopStart) };
//< loop

    unsafe { patchJump(exitJump) };
    unsafe { emitByte(OP_POP as u8) };
}
//< Jumping Back and Forth while-statement
//> Global Variables synchronize
unsafe fn synchronize() {
    unsafe { parser.panicMode = false };

    while unsafe { parser.current.r#type.clone() } as u8 != TOKEN_EOF as u8 {
        if unsafe { parser.previous.r#type.clone() } as u8 == TOKEN_SEMICOLON as u8 { return; }
        match unsafe { parser.current.r#type.clone() } {
            | TOKEN_CLASS
            | TOKEN_FUN
            | TOKEN_VAR
            | TOKEN_FOR
            | TOKEN_IF
            | TOKEN_WHILE
            | TOKEN_PRINT
            | TOKEN_RETURN
                => return,

            _ => {} // Do nothing.
        }

        unsafe { advance() };
    }
}
//< Global Variables synchronize
//> Global Variables declaration
unsafe fn declaration() {
//> match-var
    if unsafe { r#match(TOKEN_VAR) } {
        unsafe { varDeclaration() };
    } else {
        unsafe { statement() };
    }
//< match-var
/* Global Variables declaration < Global Variables match-var
    unsafe { statement() };
*/
//> call-synchronize

    if unsafe { parser.panicMode } { unsafe { synchronize() }; }
//< call-synchronize
}
//< Global Variables declaration
//> Global Variables statement
unsafe fn statement() {
    if unsafe { r#match(TOKEN_PRINT) } {
        unsafe { printStatement() };
//> Jumping Back and Forth parse-for
    } else if unsafe { r#match(TOKEN_FOR) } {
        unsafe { forStatement() };
//< Jumping Back and Forth parse-for
//> Jumping Back and Forth parse-if
    } else if unsafe { r#match(TOKEN_IF) } {
        unsafe { ifStatement() };
//< Jumping Back and Forth parse-if
//> Jumping Back and Forth parse-while
    } else if unsafe { r#match(TOKEN_WHILE) } {
        unsafe { whileStatement() };
//< Jumping Back and Forth parse-while
//> Local Variables parse-block
    } else if unsafe { r#match(TOKEN_LEFT_BRACE) } {
        unsafe { beginScope() };
        unsafe { block() };
        unsafe { endScope() };
//< Local Variables parse-block
//> parse-expressions-statement
    } else {
        unsafe { expressionStatement() };
//< parse-expressions-statement
    }
}
//< Global Variables statement

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
//> Local Variables compiler
    let mut compiler: *mut Compiler = unsafe { &mut uninit::<Compiler>() } as *mut Compiler;
    unsafe { initCompiler(compiler) };
//< Local Variables compiler
//> Compiling Expressions init-compile-chunk
    unsafe { compilingChunk = chunk };
//< Compiling Expressions init-compile-chunk
//> Compiling Expressions compile-chunk
//> init-parser-error

    unsafe { parser.hadError = false };
    unsafe { parser.panicMode = false };

//< init-parser-error
    unsafe { advance() };
//< Compiling Expressions compile-chunk
/* Compiling Expressions compile-chunk < Global Variables compile
    unsafe { expression() };
    unsafe { consume(TOKEN_EOF, "Expect end of expression.") };
*/
//> Global Variables compile

    while !unsafe { r#match(TOKEN_EOF) } {
        unsafe { declaration() };
    }

//< Global Variables compile
//> Compiling Expressions finish-compile
    unsafe { endCompiler() };
//< Compiling Expressions finish-compile
//> Compiling Expressions return-had-error
    return !unsafe { parser.hadError };
//< Compiling Expressions return-had-error
}
