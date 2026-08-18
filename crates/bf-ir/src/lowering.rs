use bf_frontend::ast::{AST, ASTNode, ASTNodeKind};

use crate::inst::{Inst, Label};

pub struct Lowerer {
    label_count: usize,
    insts: Vec<Inst>,
}

impl Default for Lowerer {
    fn default() -> Self {
        Self::new()
    }
}

impl Lowerer {
    pub fn new() -> Self {
        Self {
            label_count: 0,
            insts: Vec::new(),
        }
    }
}

impl Lowerer {
    fn new_label(&mut self) -> Label {
        let id = self.label_count;
        self.label_count += 1;

        Label(id)
    }
}

impl Lowerer {
    pub fn build(mut self, ast: &AST) -> Option<Vec<Inst>> {
        self.lower(ast.nodes())?;
        Some(self.insts)
    }

    fn lower(&mut self, nodes: &[ASTNode]) -> Option<()> {
        for n in nodes {
            match n.kind() {
                ASTNodeKind::Add => self.insts.push(Inst::Add(1)),
                ASTNodeKind::Sub => self.insts.push(Inst::Add(-1)),
                ASTNodeKind::MoveLeft => self.insts.push(Inst::Move(-1)),
                ASTNodeKind::MoveRight => self.insts.push(Inst::Move(1)),
                ASTNodeKind::Input => self.insts.push(Inst::Input),
                ASTNodeKind::Output => self.insts.push(Inst::Output),

                ASTNodeKind::Loop { body } => {
                    let loop_start = self.new_label();
                    let loop_end = self.new_label();

                    self.insts.push(Inst::Label(loop_start));

                    self.insts.push(Inst::JumpIfZero(loop_end));
                    self.lower(body)?;
                    self.insts.push(Inst::Jump(loop_start));

                    self.insts.push(Inst::Label(loop_end));
                }

                _ => return None,
            }
        }

        Some(())
    }
}
