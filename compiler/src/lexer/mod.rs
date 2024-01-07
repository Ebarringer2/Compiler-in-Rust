#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    EndofFile
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
            if c == b' ' {
                self.position += 1;
            } else if c.is_ascii_digit() {
                let start = self.position;
                while let Some(&digit) = self.input.get(self.position) {
                    if digit.is_ascii_digit() {
                        self.position += 1
                    } else {
                        break;
                    }
                }
                let number_slice: &[u8] = &self.input[start..self.position];
                let number_str: &str = std::str::from_utf8(number_slice).unwrap_or("0");
                let number = number_str.parse::<i64>().unwrap_or(0);
                return Token::Number(number);
            } else {
                self.position += 1;
                match c {
                    b'+' => return Token::Plus,
                    b'-' => return Token::Minus,
                    _ => panic!("Invalid character: {}", c as char),
                }
            }
        }
        Token::EndofFile
    }
    pub fn analyze(&mut self) {
        loop {
            let token: &mut Token = &mut self.next_token();
            println!("{:?}", token);
            if let Token::EndofFile = token {
                break;
            }
        }
    }
}