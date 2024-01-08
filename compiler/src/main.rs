mod file_map;
mod lexer;
mod parser;
mod backend_code_generation;

use file_map::FileMap;
use lexer::Lexer;
use parser::Parser;
use backend_code_generation::generate_asm;

fn main() {
    let fm: Result<FileMap, std::io::Error> = FileMap::new("C:/Users/ellio/OneDrive/Documents/GitHub/Compiler-in-Rust/compiler/src/hello.txt");
    let file_map: FileMap = fm.expect("Failed to create File Map");
    let mut lexer: Lexer = Lexer::new(file_map.as_slice());
    println!("File contents: {:?}", file_map.as_slice());
    let tokens: Vec<lexer::Token> =  lexer.analyze();
    let mut parser: Parser = Parser::new(tokens.as_slice());
    parser.parse();
    let ast: &Vec<parser::Expr> = parser.get_ast();
    println!("Fetched AST: {:?}", ast);
    let asm: String = generate_asm(ast);
    println!("ASM: \n{}", asm);
}