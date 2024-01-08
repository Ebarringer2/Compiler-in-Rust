use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    ast: Vec<Expr>
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Token, Box<Expr>)
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            ast: Vec::new()
        }
    }
    fn parse_number(&mut self, ast: &mut Vec<Expr>) -> Expr {
        if let Some(&Token::Number(value)) = self.tokens.get(self.current) {
            self.current += 1;
            let expr: Expr = Expr::Number(value);
            ast.push(expr.clone());
            expr
        } else {
            panic!("Expected a number")
        }
    }
    fn parse_term(&mut self, ast: &mut Vec<Expr>) {
        self.parse_number(ast);
        while let Some(&token) = self.tokens.get(self.current) {
            match token {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    self.current += 1;
                    let temp_expr: Expr = ast.pop().unwrap();
                    let right_expr: Expr = self.parse_number(ast);
                    let operator: Token = token.clone();
                    ast.push(Expr::BinOp(Box::new(temp_expr), operator, Box::new(right_expr)));
                }
                _ => break
            }
        }
    }
    pub fn parse(&mut self) {
        let mut ast: Vec<_> = Vec::new();
        self.parse_term(&mut ast);
        println!("Parsed AST: {:?}", self.ast);
    }
}