use std::{error::Error, fmt};

use bf_frontend::{
    lexer::{Lexer, Token},
    parser::Parser,
};
use bf_ir::ir::{IR, IRError};

use crate::c_func::CFunction;

#[derive(Debug, Clone, Copy)]
pub struct CompileOptions {
    pub opt_level: i32,
    pub boundary_check: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            opt_level: 1,
            boundary_check: true,
        }
    }
}

#[derive(Debug)]
pub enum CompileError {
    InvalidFunctionName(String),
    IR(IRError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFunctionName(name) => {
                write!(f, "invalid C function name: {name:?}")
            }
            Self::IR(IRError::BadAST(_)) => write!(f, "invalid Brainfuck syntax"),
            Self::IR(IRError::UnsupportedOptLevel(level)) => {
                write!(f, "unsupported optimization level: {level}")
            }
        }
    }
}

impl Error for CompileError {}

pub fn compile_function(
    src: &str,
    func_name: &str,
    options: CompileOptions,
) -> Result<CFunction, CompileError> {
    if !is_valid_c_function_name(func_name) {
        return Err(CompileError::InvalidFunctionName(func_name.into()));
    }

    let lexer = Lexer::new(src);
    let tokens: Vec<Token> = lexer.collect();

    let parser = Parser::new(&tokens);
    let ast = parser.parse();

    let ir = IR::new(ast, options.opt_level).map_err(|e| CompileError::IR(e))?;

    Ok(CFunction::new(func_name, ir, options.boundary_check))
}

fn is_valid_c_function_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }

    chars.all(|c| c == '_' || c.is_ascii_alphanumeric()) && !is_c_keyword(name)
}

fn is_c_keyword(name: &str) -> bool {
    matches!(
        name,
        "alignas"
            | "alignof"
            | "auto"
            | "bool"
            | "break"
            | "case"
            | "char"
            | "const"
            | "constexpr"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "inline"
            | "int"
            | "long"
            | "nullptr"
            | "register"
            | "restrict"
            | "return"
            | "short"
            | "signed"
            | "sizeof"
            | "static"
            | "static_assert"
            | "struct"
            | "switch"
            | "thread_local"
            | "true"
            | "typedef"
            | "typeof"
            | "typeof_unqual"
            | "union"
            | "unsigned"
            | "void"
            | "volatile"
            | "while"
            | "_Alignas"
            | "_Alignof"
            | "_Atomic"
            | "_BitInt"
            | "_Bool"
            | "_Complex"
            | "_Decimal128"
            | "_Decimal32"
            | "_Decimal64"
            | "_Generic"
            | "_Imaginary"
            | "_Noreturn"
            | "_Static_assert"
            | "_Thread_local"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_a_function() {
        let function = compile_function("+.", "bf_main", CompileOptions::default()).unwrap();

        assert_eq!(function.name(), "bf_main");
    }

    #[test]
    fn rejects_invalid_c_function_names() {
        for name in ["", "2bad", "has-dash", "while"] {
            assert!(matches!(
                compile_function("", name, CompileOptions::default()),
                Err(CompileError::InvalidFunctionName(_))
            ));
        }
    }
}
