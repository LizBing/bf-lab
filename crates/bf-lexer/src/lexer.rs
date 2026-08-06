use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    raw: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(raw: &'a [u8]) -> Self {
        Self {
            raw,
            cursor: 0
        }
    }
}

impl Lexer<'_> {
    pub fn next(&mut self) -> Option<Token> {
        let cur = self.cursor;
        self.cursor += 1;
        
        let c = self.raw.get(cur)?;
        match *c {
            b'+' => Some(Token {
                kind: TokenKind::Add,
                loc: cur
            }),

            b'-' => Some(Token {
                kind: TokenKind::Sub,
                loc: cur
            }),

            b'<' => Some(Token {
                kind: TokenKind::MoveLeft,
                loc: cur
            }),

            b'>' => Some(Token {
                kind: TokenKind::MoveRight,
                loc: cur
            }),

            b'[' => Some(Token {
                kind: TokenKind::LoopStart,
                loc: cur
            }),

            b']' => Some(Token {
                kind: TokenKind::LoopEnd,
                loc: cur
            }),

            b',' => Some(Token {
                kind: TokenKind::Input,
                loc: cur
            }),

            b'.' => Some(Token {
                kind: TokenKind::Output,
                loc: cur
            }),

            _ => unreachable!()
        }
    }
}
