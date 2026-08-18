use std::path::Path;

use bf_frontend::{lexer::Lexer, parser::Parser};
use bf_ir::ir::IR;

use crate::{c_func::CFunction, code_file::CodeFile};

mod c_func;
mod code_file;
mod inst_translator;

#[test]
fn test_hello_world() {
    let bf_code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";

    let lexer = Lexer::new(bf_code);
    let tokens: Vec<_> = lexer.collect();

    let parser = Parser::new(&tokens);
    let ast = parser.parse();

    let ir = IR::new(ast, 1).unwrap();

    let func = CFunction::new("hello_world", ir.clone(), true);
    let mut cf = CodeFile::new();
    assert!(cf.add_bf_func(func));
    
    let func = CFunction::new("hello", ir.clone(), true);
    assert!(cf.add_bf_func(func));
    
    let path = Path::new("hello_world.c");
    cf.write_to_disk(path).unwrap();
}
