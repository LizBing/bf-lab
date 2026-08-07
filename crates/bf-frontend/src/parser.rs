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
    fn parse_add(&mut self, start_loc: usize) -> Option<ASTNode> {
        let mut last_loc = start_loc;

        let mut value = 0;
        
        loop {
            if let Some(t) = self.peek() {
                match t.kind() {
                    TokenKind::Add => value += 1,
                    TokenKind::Sub => value -= 1,

                    _ => break,
                }

                last_loc = t.loc();
                self.step();
            } else { break }
        }

        if value == 0 {
            None
        } else {
            let span = Span {
                start: start_loc,
                end: last_loc + 1,
            };
            
            Some(ASTNode {
                kind: ASTNodeKind::Add(value),
                span
            })
        }
    }

    fn parse_move(&mut self, start_loc: usize) -> Option<ASTNode> {
        let mut last_loc = start_loc;
        
        let mut value = 0;
        
        loop {
            if let Some(t) = self.peek() {
                match t.kind() {
                    TokenKind::MoveLeft => value -= 1,
                    TokenKind::MoveRight => value += 1,

                    _ => break,
                }

                last_loc = t.loc();
                self.step();
            } else { break }
        }

        if value == 0 {
            None
        } else {
            let span = Span {
                start: start_loc,
                end: last_loc + 1,
            };
            
            Some(ASTNode {
                kind: ASTNodeKind::Move(value),
                span
            })
        }
    }

    fn parse_input(&mut self, loc: usize) -> ASTNode {
        self.step();

        let span = Span {
            start: loc,
            end: loc,
        };

        ASTNode { kind: ASTNodeKind::Input, span }
    }

    fn parse_output(&mut self, loc: usize) -> ASTNode {
        self.step();

        let span = Span {
            start: loc,
            end: loc + 1,
        };

        ASTNode { kind: ASTNodeKind::Output, span }
    }

    fn parse_loop(&mut self, start_loc: usize) -> ASTNode {
        let mut last_loc = start_loc;

        self.step();

        let mut nodes = Vec::new();

        let is_closed;
        loop {
            if let Some(t) = self.peek() {
                last_loc = t.loc();
                
                let node = match t.kind() {
                    TokenKind::Add | TokenKind::Sub => self.parse_add(t.loc()),
                    TokenKind::MoveLeft | TokenKind::MoveRight => self.parse_move(t.loc()),
                    TokenKind::Input => Some(self.parse_input(t.loc())),
                    TokenKind::Output => Some(self.parse_output(t.loc())),

                    TokenKind::LoopStart => Some(self.parse_loop(t.loc())),
                    TokenKind::LoopEnd  => {
                        is_closed = true;
                        break;
                    }
                };

                if let Some(n) = node {
                    nodes.push(n);
                }
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
            ASTNode { kind: ASTNodeKind::Loop(nodes), span }
        } else {
            ASTNode { kind: ASTNodeKind::Invalid(InvalidNode::OpenLoopStart), span }
        }
    }

    fn parse_open_loop_end(&mut self, loc: usize) -> ASTNode {
        self.step();

        let span = Span {
            start: loc,
            end: loc + 1,
        };

        ASTNode { kind: ASTNodeKind::Invalid(InvalidNode::OpenLoopEnd), span }
    }
    
    pub fn parse(mut self) -> AST {
        let mut nodes = Vec::new();

        loop {
            if let Some(t) = self.peek() {
                let node = match t.kind() {
                    TokenKind::Add | TokenKind::Sub => self.parse_add(t.loc()),
                    TokenKind::MoveLeft | TokenKind::MoveRight => self.parse_move(t.loc()),
                    TokenKind::Input => Some(self.parse_input(t.loc())),
                    TokenKind::Output => Some(self.parse_output(t.loc())),

                    TokenKind::LoopStart => Some(self.parse_loop(t.loc())),
                    TokenKind::LoopEnd => Some(self.parse_open_loop_end(t.loc()))
                };

                if let Some(n) = node {
                    nodes.push(n);
                }
            } else {
                break;
            }
        }
        
        AST { nodes }
    }
}
