use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    ast: Vec<Expr>
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Operator, Box<Expr>)
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exp
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            ast: Vec::new()
        }
    }
    fn parse_number(&mut self) -> Expr {
        if let Some(&Token::Number(value)) = self.tokens.get(self.current) {
            self.current += 1;
            let expr: Expr = Expr::Number(value);
            self.ast.push(expr.clone());
            expr
        } else {
            panic!("Expected a number")
        }
    }
    fn parse_term(&mut self) {
        self.parse_factor();
        while let Some(&token) = self.tokens.get(self.current) {
            match token {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Exp => {
                    self.current += 1;
                    let temp_expr: Expr = self.ast.pop().unwrap();
                    let right_expr: Expr = self.parse_factor();
                    let operator: Operator = match token {
                        Token::Plus => Operator::Add,
                        Token::Minus => Operator::Subtract,
                        Token::Multiply => Operator::Multiply,
                        Token::Divide => Operator::Divide,
                        Token::Exp => Operator::Exp,
                        _ => unreachable!()
                    };
                    self.ast.push(Expr::BinOp(Box::new(temp_expr), operator, Box::new(right_expr)));
                }
                _ => break
            }
        }
    }
    fn parse_factor(&mut self) -> Expr {
        self.parse_number()
    }
    pub fn parse(&mut self) {
        self.parse_term();
    }
    pub fn get_ast(&self) -> &Vec<Expr> {
        return &self.ast;
    }
}