use bf_frontend::{lexer::{Lexer, Token}, parser::Parser};
use bf_ir::ir::{IR, IRError};

use crate::c_func::CFunction;

pub struct CompileOptions {
    pub opt_level: i32,
    pub boundary_check: bool,
}

#[derive(Debug)]
pub enum CompileError {
    IR(IRError),
}

pub fn compile_func(src: &str, func_name: &str, options: CompileOptions) -> Result<CFunction, CompileError> {
    let lexer = Lexer::new(src);
    let tokens: Vec<Token> = lexer.collect();

    let parser = Parser::new(&tokens);
    let ast = parser.parse();

    let ir = IR::new(ast, options.opt_level).map_err(|e| CompileError::IR(e))?;

    Ok(CFunction::new(func_name, ir, options.boundary_check))
}
