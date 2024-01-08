const EXP: u8 = b'^';

#[derive(Debug)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Exp,
    EndofFile
}

impl Copy for Token { }

impl Clone for Token {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer object
    /// which is used to break down an
    /// 
    /// inputted slice of bytes and digest it into Tokens for 
    /// later analysis
    pub fn new(input: &'a [u8]) -> Self {
        Lexer { input, position: 0}
    }
    fn next_token(&mut self) -> Token {
        while let Some(&c) = self.input.get(self.position) {
            match c {
                b' ' => self.position += 1,
                b'0'..=b'9' => {
                    let start = self.position;
                    while let Some(&digit) = self.input.get(self.position) {
                        if digit.is_ascii_digit() {
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                    let number_slice: &[u8] = &self.input[start..self.position];
                    if self.input.get(self.position) == Some(&EXP) {
                        self.position += 1;
                        return Token::Exp
                    } else if Some(&b'*') == self.input.get(self.position) {
                        self.position += 1;
                        return Token::Multiply;
                    } else {
                        let number_str: &str = std::str::from_utf8(number_slice).unwrap_or("0");
                        let number: i64 = number_str.parse::<i64>().unwrap_or(0);
                        return Token::Number(number);
                    }
                }
                43 => {
                    self.position += 1;
                    return Token::Plus
                }
                45 => {
                    self.position += 1;
                    return Token::Minus;
                }
                47 => {
                    self.position += 1;
                    return Token::Divide;
                }
                EXP => {
                    self.position += 1;
                    return Token::Exp;
                }
                42 => {
                    self.position += 1;
                    return Token::Multiply
                }
                _ => panic!("invalid character: {}", c as char)
            }
        }
        return Token::EndofFile;
    }
    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens: Vec<_> = Vec::new();
        loop {
            let token: &mut Token = &mut self.next_token();
            println!("{:?}", token);
            tokens.push(token.clone());
            if let Token::EndofFile = token {
                break;
            }
        }
        return tokens;
    }
}