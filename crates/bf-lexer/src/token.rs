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
    pub(crate) kind: TokenKind,
    pub(crate) loc: usize,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn loc(&self) -> usize {
        self.loc
    }
}
