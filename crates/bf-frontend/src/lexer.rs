use std::str::CharIndices;

// + - < > [ ] , .
#[derive(Clone, Copy)]
pub enum TokenKind {
    Add,
    Sub,
    MoveLeft,
    MoveRight,
    LoopStart,
    LoopEnd,
    Input,
    Output,
}

pub struct Token {
    kind: TokenKind,
    loc: usize,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn loc(&self) -> usize {
        self.loc
    }
}

pub struct Lexer<'a> {
    iter: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            iter: raw.char_indices(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (offs, c) = self.iter.next()?;

            let res = match c {
                '+' => Token {
                    kind: TokenKind::Add,
                    loc: offs,
                },

                '-' => Token {
                    kind: TokenKind::Sub,
                    loc: offs,
                },

                '<' => Token {
                    kind: TokenKind::MoveLeft,
                    loc: offs,
                },

                '>' => Token {
                    kind: TokenKind::MoveRight,
                    loc: offs,
                },

                '[' => Token {
                    kind: TokenKind::LoopStart,
                    loc: offs,
                },

                ']' => Token {
                    kind: TokenKind::LoopEnd,
                    loc: offs,
                },

                ',' => Token {
                    kind: TokenKind::Input,
                    loc: offs,
                },

                '.' => Token {
                    kind: TokenKind::Output,
                    loc: offs,
                },

                _ => continue,
            };

            return Some(res);
        }
    }
}
