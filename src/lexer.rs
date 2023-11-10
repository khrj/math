#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Rbrace,
    Lbrace,
    Eof,
}

const NUMBER_LIST: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Lexer {
    tokens: Vec<Token>,
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            tokens: vec![],
            position: 0,
        }
    }

    fn next(&mut self) {
        self.position += 1
    }

    fn current_char(&self) -> Option<&char> {
        self.input.get(self.position)
    }

    pub fn get_tokens(&mut self) -> Result<&Vec<Token>, String> {
        loop {
            match self.current_char() {
                None => {
                    self.tokens.push(Token::Eof);
                    break;
                }
                Some(c) => {
                    let token = match c {
                        ' ' | '\t' | '\n' => {
                            self.next();
                            continue;
                        }
                        '+' => Token::Add,
                        '-' => Token::Subtract,
                        '*' => Token::Multiply,
                        '/' => Token::Divide,
                        '(' | '[' | '{' => Token::Lbrace,
                        ')' | ']' | '}' => Token::Rbrace,
                        _ => {
                            if NUMBER_LIST.contains(&c) {
                                let token = self.generate_number();
                                self.tokens.push(token?);
                                continue;
                            } else {
                                return Err(format!("{} is unsupported", c));
                            }
                        }
                    };

                    self.tokens.push(token);
                    self.next()
                }
            }
        }

        return Ok(&self.tokens);
    }

    fn generate_number(&mut self) -> Result<Token, String> {
        let mut decimal_count = 0;
        let mut string = String::new();

        loop {
            match self.current_char() {
                Some(current_input) => {
                    if NUMBER_LIST.contains(&current_input) {
                        if *current_input == '.' && decimal_count == 0 {
                            decimal_count += 1
                        }

                        if string.len() == 0 && decimal_count > 0 {
                            // Decimal character is first place, like in .1415
                            // = 0.1415

                            string.push('0')
                        }

                        string.push(*current_input);
                        self.next()
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        match string.parse::<f64>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(format!("Couldn't parse {} as number", string)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn all_tokens() {
        let mut my_lexer = Lexer::new("{[( )]} 4646 + - * / 565.788");
        assert_eq!(
            "[Lbrace, Lbrace, Lbrace, Rbrace, Rbrace, Rbrace, Number(4646.0), Add, Subtract, Multiply, Divide, Number(565.788), Eof]", 
            format!("{:?}", my_lexer.get_tokens().unwrap())
        );
    }

    #[test]
    fn invalid_characters() {
        let mut my_lexer = Lexer::new("$#@");
        assert!(my_lexer.get_tokens().is_err())
    }

    #[test]
    fn bad_decimal() {
        let mut my_lexer = Lexer::new("2.2.2");
        assert!(my_lexer.get_tokens().is_err())
    }
}
