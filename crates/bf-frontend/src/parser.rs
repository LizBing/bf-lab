use crate::{ast::{AST, ASTNode, ASTNodeKind, InvalidNode, Span}, lexer::{Token, TokenKind}};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn step(&mut self) {
        debug_assert!(self.peek().is_some());
        self.pos += 1;
    }
}

impl Parser {
    fn parse_one(&mut self, loc: usize, kind: ASTNodeKind) -> ASTNode {
        self.step();

        let span = Span {
            start: loc,
            end: loc + 1,
        };

        ASTNode { kind, span }
    }
    
    fn parse_loop(&mut self, start_loc: usize) -> ASTNode {
        self.step();
        
        let mut last_loc = start_loc;
        let mut body = Vec::new();

        let is_closed;
        loop {
            if let Some(t) = self.peek() {
                last_loc = t.loc();
                
                let n = match t.kind() {
                    TokenKind::Add => self.parse_one(last_loc, ASTNodeKind::Add),
                    TokenKind::Sub => self.parse_one(last_loc, ASTNodeKind::Sub),
                    TokenKind::MoveLeft => self.parse_one(last_loc, ASTNodeKind::MoveLeft),
                    TokenKind::MoveRight => self.parse_one(last_loc, ASTNodeKind::MoveRight),
                    TokenKind::Input => self.parse_one(last_loc, ASTNodeKind::Input),
                    TokenKind::Output => self.parse_one(last_loc, ASTNodeKind::Output),
                    
                    TokenKind::LoopStart => self.parse_loop(last_loc),
                    TokenKind::LoopEnd  => {
                        is_closed = true;
                        break;
                    }
                };

                body.push(n);
            } else {
                is_closed = false;
                break;
            }
        }

        let span = Span {
            start: start_loc,
            end: last_loc + 1
        };
        
        if is_closed {
            ASTNode { kind: ASTNodeKind::Loop { body }, span }
        } else {
            ASTNode { kind: ASTNodeKind::Invalid(InvalidNode::OpenLoopStart), span }
        }
    }
    
    pub fn parse(mut self) -> AST {
        let mut nodes = Vec::new();

        loop {
            if let Some(t) = self.peek() {
                let loc = t.loc();
                
                let n = match t.kind() {
                    TokenKind::Add => self.parse_one(loc, ASTNodeKind::Add),
                    TokenKind::Sub => self.parse_one(loc, ASTNodeKind::Sub),
                    TokenKind::MoveLeft => self.parse_one(loc, ASTNodeKind::MoveLeft),
                    TokenKind::MoveRight => self.parse_one(loc, ASTNodeKind::MoveRight),
                    TokenKind::Input => self.parse_one(loc, ASTNodeKind::Input),
                    TokenKind::Output => self.parse_one(loc, ASTNodeKind::Output),
                    
                    TokenKind::LoopStart => self.parse_loop(loc),
                    TokenKind::LoopEnd  => self.parse_one(loc, ASTNodeKind::Invalid(InvalidNode::OpenLoopEnd)),
                };

                nodes.push(n);
            } else {
                break;
            }
        }
        
        AST { nodes }
    }
}
