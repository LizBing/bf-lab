use bf_frontend::ast::{AST, ASTNode, ASTNodeKind};

use crate::inst::Inst;

pub struct Program {
    ir: Vec<Inst>
}

impl Program {
    pub fn new(ast: AST) -> Option<Self> {
        let mut ir = Vec::new();
        Self::lower(ast.nodes(), &mut ir)?;
        
        Some(Self { ir })
    }

    fn lower(nodes: &[ASTNode], ir: &mut Vec<Inst>) -> Option<()> {
        for n in nodes {
            match n.kind() {
                ASTNodeKind::Add(n) => ir.push(Inst::Add(*n)),
                ASTNodeKind::Move(n) => ir.push(Inst::Move(*n)),
                ASTNodeKind::Input => ir.push(Inst::Input),
                ASTNodeKind::Output => ir.push(Inst::Output),

                ASTNodeKind::Loop { body } => {
                    let jump_if_zero_pos = ir.len();
                    ir.push(Inst::JumpIfZero(0));
                    
                    let loop_start = ir.len();
                    Self::lower(body, ir)?;
                    ir.push(Inst::Jump(loop_start));

                    let loop_end = ir.len();
                    ir[jump_if_zero_pos] = Inst::JumpIfZero(loop_end);
                }

                _ => return None,
            }
        }

        Some(())
    }
}

impl Program {
    pub fn ir(&self) -> &Vec<Inst> {
        &self.ir
    }
}
