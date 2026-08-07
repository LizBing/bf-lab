#[derive(Debug)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug)]
pub enum ASTNodeKind {
    Add(i8),
    Move(i32),
    Input,
    Output,
    Loop { body: Vec<ASTNode> },
    Invalid(InvalidNode)
}

#[derive(Debug)]
pub struct ASTNode {
    pub(crate) kind: ASTNodeKind,
    pub(crate) span: Span,
}

impl ASTNode {
    pub fn kind(&self) -> &ASTNodeKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub enum InvalidNode {
    OpenLoopStart,
    OpenLoopEnd,
}

#[derive(Debug)]
pub struct AST {
    pub(crate) nodes: Vec<ASTNode>
}

impl AST {
    pub fn nodes(&self) -> &Vec<ASTNode> {
        &self.nodes
    }
}
