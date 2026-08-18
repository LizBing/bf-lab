pub mod ast;
pub mod lexer;
pub mod parser;

#[test]
fn test_correct() {
    let code = "+++----[-]";

    let l = lexer::Lexer::new(code);
    let tokens: Vec<_> = l.collect();

    let p = parser::Parser::new(&tokens);

    let ast = p.parse();

    println!("{:?}", ast)
}
