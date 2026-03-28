use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Module,
    Func,
    Input,
    Output,
    Effect,
    Begin,
    End,
    Return,
    If,
    Then,
    Else,
    While,
    Match,
    Case,
    Pure,
    Io,
    Alloc,
    Unsafe,
    Syscall,
    True,
    False,
    OwnKw,
    ViewKw,
    OptionKw,
    ResultKw,
    SomeKw,
    NoneKw,
    OkKw,
    ErrKw,
    Ident(String),
    Int(String),
    Colon,
    Dot,
    Comma,
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Lt,
    Le,
    Gt,
    Ge,
    EqEq,
    Ne,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character `{ch}` at byte {offset}")]
    UnexpectedChar { ch: char, offset: usize },
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let bytes = input.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let ch = input[i..].chars().next().expect("valid char boundary");
        match ch {
            ' ' | '\t' | '\r' => i += ch.len_utf8(),
            '\n' => { out.push(Token { kind: TokenKind::Newline, offset: i }); i += 1; }
            '#' => { while i < bytes.len() && bytes[i] != b'\n' { i += 1; } }
            ':' => { out.push(Token { kind: TokenKind::Colon, offset: i }); i += 1; }
            '.' => { out.push(Token { kind: TokenKind::Dot, offset: i }); i += 1; }
            ',' => { out.push(Token { kind: TokenKind::Comma, offset: i }); i += 1; }
            '(' => { out.push(Token { kind: TokenKind::LParen, offset: i }); i += 1; }
            ')' => { out.push(Token { kind: TokenKind::RParen, offset: i }); i += 1; }
            '+' => { out.push(Token { kind: TokenKind::Plus, offset: i }); i += 1; }
            '-' => { out.push(Token { kind: TokenKind::Minus, offset: i }); i += 1; }
            '*' => { out.push(Token { kind: TokenKind::Star, offset: i }); i += 1; }
            '/' => { out.push(Token { kind: TokenKind::Slash, offset: i }); i += 1; }
            '<' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    out.push(Token { kind: TokenKind::Le, offset: i }); i += 2;
                } else {
                    out.push(Token { kind: TokenKind::Lt, offset: i }); i += 1;
                }
            }
            '>' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    out.push(Token { kind: TokenKind::Ge, offset: i }); i += 2;
                } else {
                    out.push(Token { kind: TokenKind::Gt, offset: i }); i += 1;
                }
            }
            '=' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    out.push(Token { kind: TokenKind::EqEq, offset: i }); i += 2;
                } else {
                    return Err(LexError::UnexpectedChar { ch, offset: i });
                }
            }
            '!' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    out.push(Token { kind: TokenKind::Ne, offset: i }); i += 2;
                } else {
                    return Err(LexError::UnexpectedChar { ch, offset: i });
                }
            }
            '0'..='9' => {
                let start = i;
                while i < bytes.len() && input.as_bytes()[i].is_ascii_digit() { i += 1; }
                out.push(Token { kind: TokenKind::Int(input[start..i].to_string()), offset: start });
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                let start = i;
                while i < bytes.len() {
                    let b = input.as_bytes()[i];
                    if b.is_ascii_alphanumeric() || b == b'_' { i += 1; } else { break; }
                }
                let text = &input[start..i];
                let kind = match text {
                    "module" => TokenKind::Module,
                    "func" => TokenKind::Func,
                    "input" => TokenKind::Input,
                    "output" => TokenKind::Output,
                    "effect" => TokenKind::Effect,
                    "begin" => TokenKind::Begin,
                    "end" => TokenKind::End,
                    "return" => TokenKind::Return,
                    "if" => TokenKind::If,
                    "then" => TokenKind::Then,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "match" => TokenKind::Match,
                    "case" => TokenKind::Case,
                    "pure" => TokenKind::Pure,
                    "io" => TokenKind::Io,
                    "alloc" => TokenKind::Alloc,
                    "unsafe" => TokenKind::Unsafe,
                    "syscall" => TokenKind::Syscall,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "own" => TokenKind::OwnKw,
                    "view" => TokenKind::ViewKw,
                    "option" => TokenKind::OptionKw,
                    "result" => TokenKind::ResultKw,
                    "some" => TokenKind::SomeKw,
                    "none" => TokenKind::NoneKw,
                    "ok" => TokenKind::OkKw,
                    "err" => TokenKind::ErrKw,
                    _ => TokenKind::Ident(text.to_string()),
                };
                out.push(Token { kind, offset: start });
            }
            _ => return Err(LexError::UnexpectedChar { ch, offset: i }),
        }
    }

    out.push(Token { kind: TokenKind::Eof, offset: input.len() });
    Ok(out)
}
