use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Token, Box<Expr>)
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0}
    }
    fn parse_number(&mut self) -> Expr {
        if let Token::Number(value) = &self.tokens[self.current] {
            self.current += 1;
            Expr::Number(*value)
        } else {
            panic!("Expected a number")
        }
    }
    fn parse_term(&mut self) -> Expr {
        let mut left_expr: Expr = self.parse_number();
        while let Token::Multiply | Token::Plus = &self.tokens[self.current] {
            let operator: Token = self.tokens[self.current].clone();
            self.current += 1;
            let right_expr: Expr = self.parse_number();
            left_expr = Expr::BinOp(Box::new(left_expr), operator, Box::new(right_expr));
        }
        return left_expr
    }
    pub fn parse(&mut self) -> Expr {
        self.parse_term()
    }
}