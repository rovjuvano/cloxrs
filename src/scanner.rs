//> Scanning on Demand scanner-c
use crate::common::*;
//> Scanning on Demand scanner-h
//> token-type
#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens.
    TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
    TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
    TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,
    // One or two character tokens.
    TOKEN_BANG, TOKEN_BANG_EQUAL,
    TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER, TOKEN_GREATER_EQUAL,
    TOKEN_LESS, TOKEN_LESS_EQUAL,
    // Literals.
    TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,
    // Keywords.
    TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
    TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
    TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
    TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

    TOKEN_ERROR, TOKEN_EOF,
}
pub use TokenType::*;
//< token-type
//> token-struct

#[derive(Clone)] // Copy too but made explicit
pub struct Token {
    pub r#type: TokenType,
    pub start: *const u8,
    pub length: isize,
    pub line: isize,
}
//< token-struct

// no need to forward declare initScanner
//> scan-token-h
// no need to forward declare scanToken
//< scan-token-h
//< Scanning on Demand scanner-h

#[derive(Clone)] // Copy too but made explicit
struct Scanner {
    pub start: *const u8,
    pub current: *const u8,
    pub line: isize,
}

static mut scanner: Scanner = unsafe { uninit_static!(Scanner) };
//> init-scanner
pub unsafe fn initScanner(mut source: *const u8) {
    unsafe { scanner.start = source };
    unsafe { scanner.current = source };
    unsafe { scanner.line = 1 };
}
//< init-scanner
//> is-alpha
fn isAlpha(mut c: u8) -> bool {
    return (c >= b'a' && c <= b'z') ||
           (c >= b'A' && c <= b'Z') ||
            c == b'_';
}
//< is-alpha
//> is-digit
fn isDigit(mut c: u8) -> bool {
    return c >= b'0' && c <= b'9';
}
//< is-digit
//> is-at-end
unsafe fn isAtEnd() -> bool {
    return unsafe { *scanner.current } == b'\0';
}
//< is-at-end
//> advance
unsafe fn advance() -> u8 {
    unsafe { scanner.current = unsafe { scanner.current.offset(1) } };
    return unsafe { *scanner.current.offset(-1) };
}
//< advance
//> peek
unsafe fn peek() -> u8 {
    return unsafe { *scanner.current };
}
//< peek
//> peek-next
unsafe fn peekNext() -> u8 {
    if unsafe { isAtEnd() } { return b'\0'; }
    return unsafe { *scanner.current.offset(1) };
}
//< peek-next
//> match
unsafe fn r#match(mut expected: u8) -> bool {
    if unsafe { isAtEnd() } { return false; }
    if unsafe { *scanner.current } != expected { return false; }
    unsafe { scanner.current = unsafe { scanner.current.offset(1) } };
    return true;
}
//< match
//> make-token
unsafe fn makeToken(mut r#type: TokenType) -> Token {
    let mut token: Token = unsafe { uninit::<Token>() };
    token.r#type = r#type.clone();
    token.start = unsafe { scanner.start };
    token.length = unsafe { scanner.current.offset_from(unsafe { scanner.start }) };
    token.line = unsafe { scanner.line };
    token
}
//< make-token
//> error-token
unsafe fn errorToken(mut message: &str) -> Token {
    let mut token: Token = unsafe { uninit::<Token>() };
    token.r#type = TOKEN_ERROR;
    token.start = message.as_ptr();
    token.length = message.len() as isize;
    token.line = unsafe { scanner.line };
    return token;
}
//< error-token
//> skip-whitespace
unsafe fn skipWhitespace() {
    loop {
        let mut c: u8 = unsafe { peek() };
        match c {
            | b' '
            | b'\r'
            | b'\t' => {
                let _ = unsafe { advance() };
            }
//> newline
            b'\n' => {
                unsafe { scanner.line += 1 };
                let _ = unsafe { advance() };
            }
//< newline
//> comment
            b'/' => {
                if unsafe { peekNext() } == b'/' {
                    // A comment goes until the end of the line.
                    while unsafe { peek() } != b'\n' && !unsafe { isAtEnd() } { let _ = unsafe { advance() }; }
                } else {
                    return;
                }
            }
//< comment
            _ => return,
        }
    }
}
//< skip-whitespace
//> check-keyword
unsafe fn checkKeyword(mut start: isize, mut length: isize,
        mut rest: &str, mut r#type: TokenType) -> TokenType {
    if unsafe { scanner.current.offset_from(scanner.start) } == start + length &&
            unsafe { memcmp(unsafe { scanner.start.offset(start) }, rest.as_ptr(), length as usize) } == 0 {
        return r#type;
    }

    return TOKEN_IDENTIFIER;
}
//< check-keyword
//> identifier-type
unsafe fn identifierType() -> TokenType {
//> keywords
    match unsafe { *scanner.start } {
        b'a' => return unsafe { checkKeyword(1, 2, "nd", TOKEN_AND) },
        b'c' => return unsafe { checkKeyword(1, 4, "lass", TOKEN_CLASS) },
        b'e' => return unsafe { checkKeyword(1, 3, "lse", TOKEN_ELSE) },
//> keyword-f
        b'f' => {
            if unsafe { scanner.current.offset_from(unsafe { scanner.start }) } > 1 {
                match unsafe { *scanner.start.offset(1) } {
                    b'a' => return unsafe { checkKeyword(2, 3, "lse", TOKEN_FALSE) },
                    b'o' => return unsafe { checkKeyword(2, 1, "r", TOKEN_FOR) },
                    b'u' => return unsafe { checkKeyword(2, 1, "n", TOKEN_FUN) },
                    _ => {}
                }
            }
        }
//< keyword-f
        b'i' => return unsafe { checkKeyword(1, 1, "f", TOKEN_IF) },
        b'n' => return unsafe { checkKeyword(1, 2, "il", TOKEN_NIL) },
        b'o' => return unsafe { checkKeyword(1, 1, "r", TOKEN_OR) },
        b'p' => return unsafe { checkKeyword(1, 4, "rint", TOKEN_PRINT) },
        b'r' => return unsafe { checkKeyword(1, 5, "eturn", TOKEN_RETURN) },
        b's' => return unsafe { checkKeyword(1, 4, "uper", TOKEN_SUPER) },
//> keyword-t
        b't' => {
            if unsafe { scanner.current.offset_from(unsafe { scanner.start }) } > 1 {
                match unsafe { *scanner.start.offset(1) } {
                    b'h' => return unsafe { checkKeyword(2, 2, "is", TOKEN_THIS) },
                    b'r' => return unsafe { checkKeyword(2, 2, "ue", TOKEN_TRUE) },
                    _ => {}
                }
            }
        }
//< keyword-t
        b'v' => return unsafe { checkKeyword(1, 2, "ar", TOKEN_VAR) },
        b'w' => return unsafe { checkKeyword(1, 4, "hile", TOKEN_WHILE) },
        _ => {}
    }

//< keywords
    return TOKEN_IDENTIFIER;
}
//< identifier-type
//> identifier
unsafe fn identifier() -> Token {
    while isAlpha(unsafe { peek() }) || isDigit(unsafe { peek() }) { let _ = unsafe { advance() }; }
    return unsafe { makeToken(unsafe { identifierType() }) };
}
//< identifier
//> number
unsafe fn number() -> Token {
    while isDigit(unsafe { peek() }) { let _ = unsafe { advance() }; }

    // Look for a fractional part.
    if unsafe { peek() } == b'.' && isDigit(unsafe { peekNext() }) {
        // Consume the ".".
        let _ = unsafe { advance() };

        while isDigit(unsafe { peek() }) { let _ = unsafe { advance() }; }
    }

    return unsafe { makeToken(TOKEN_NUMBER) };
}
//< number
//> string
unsafe fn string() -> Token {
    while unsafe { peek() } != b'"' && !unsafe { isAtEnd() } {
        if unsafe { peek() } == b'\n' { unsafe { scanner.line += 1 }; }
        let _ = unsafe { advance() };
    }

    if unsafe { isAtEnd() } { return unsafe { errorToken("Unterminated string.") }; }

    // The closing quote.
    let _ = unsafe { advance() };
    return unsafe { makeToken(TOKEN_STRING) };
}
//< string
//> scan-token
pub unsafe fn scanToken() -> Token {
//> call-skip-whitespace
    unsafe { skipWhitespace() };
//< call-skip-whitespace
    unsafe { scanner.start = unsafe { scanner.current } };

    if unsafe { isAtEnd() } { return unsafe { makeToken(TOKEN_EOF) }; }
//> scan-char

    let mut c: u8 = unsafe { advance() };
//> scan-identifier
    if isAlpha(c) { return unsafe { identifier() }; }
//< scan-identifier
//> scan-number
    if isDigit(c) { return unsafe { number() }; }
//< scan-number

    match c {
        b'(' => return unsafe { makeToken(TOKEN_LEFT_PAREN) },
        b')' => return unsafe { makeToken(TOKEN_RIGHT_PAREN) },
        b'{' => return unsafe { makeToken(TOKEN_LEFT_BRACE) },
        b'}' => return unsafe { makeToken(TOKEN_RIGHT_BRACE) },
        b';' => return unsafe { makeToken(TOKEN_SEMICOLON) },
        b',' => return unsafe { makeToken(TOKEN_COMMA) },
        b'.' => return unsafe { makeToken(TOKEN_DOT) },
        b'-' => return unsafe { makeToken(TOKEN_MINUS) },
        b'+' => return unsafe { makeToken(TOKEN_PLUS) },
        b'/' => return unsafe { makeToken(TOKEN_SLASH) },
        b'*' => return unsafe { makeToken(TOKEN_STAR) },
//> two-char
        b'!' =>
            return unsafe { makeToken(
                if unsafe { r#match(b'=') } { TOKEN_BANG_EQUAL } else { TOKEN_BANG }) },
        b'=' =>
            return unsafe { makeToken(
                if unsafe { r#match(b'=') } { TOKEN_EQUAL_EQUAL } else { TOKEN_EQUAL }) },
        b'<' =>
            return unsafe { makeToken(
                if unsafe { r#match(b'=') } { TOKEN_LESS_EQUAL } else { TOKEN_LESS }) },
        b'>' =>
            return unsafe { makeToken(
                if unsafe { r#match(b'=') } { TOKEN_GREATER_EQUAL } else { TOKEN_GREATER }) },
//< two-char
//> scan-string
        b'"' => return unsafe { string() },
//< scan-string
        _ => {}
    }
//< scan-char

    return unsafe { errorToken("Unexpected character.") };
}
//< scan-token
