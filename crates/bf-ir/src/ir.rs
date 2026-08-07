use bf_frontend::ast::AST;

use crate::{inst::Inst, ir::IRError::UnsupportedOptLevel, lowering::Lowerer, optimizers::PeepholeOptimizer};

pub enum IRError {
    BadAST(AST),
    UnsupportedOptLevel(i32),
}

pub struct IR {
    insts: Vec<Inst>,
}

impl IR {
    pub fn new(ast: AST, opt_level: i32) -> Result<Self, IRError> {
        let lowerer = Lowerer::new();
        let mut insts = match lowerer.build(&ast) {
            None => return Err(IRError::BadAST(ast)),
            Some(x) => x
        };

        match opt_level {
            0 => (),

            1 => {
                let optimizer = PeepholeOptimizer::new(&insts);
                insts = optimizer.optimize();
            }

            _ => return Err(UnsupportedOptLevel(opt_level)),
        }

        Ok(Self { insts })
    }
}

impl IR {
    pub fn instructions(&self) -> &[Inst] {
        &self.insts
    }
}
