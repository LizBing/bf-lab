use bf_frontend::{lexer::Lexer, parser::Parser};
use bf_ir::ir::IR;

use crate::code_file::CodeFile;

mod code_file;
mod inst_translator;

#[test]
fn test_hello_world() {
    // let bf_code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    let bf_code = 
        "++++++++[>+++++++++<-]>.";

    let lexer = Lexer::new(bf_code);
    let tokens: Vec<_> = lexer.collect();

    let parser = Parser::new(&tokens);
    let ast = parser.parse();

    let ir = IR::new(ast, 1).unwrap();

    let mut cf = CodeFile::new();
    cf.add_bf_func("foo".into(), ir, true);

    cf.write_to_disk("./hello_world.c").unwrap();
}
