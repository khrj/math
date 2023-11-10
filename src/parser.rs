use std::borrow::Borrow;
use std::rc::Rc;

use crate::ast::{self, Evaluable};
use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) {
        self.position += 1;
    }

    pub fn parse_expr(&mut self) -> ast::Node {
        let mut result = self.factor();

        loop {
            match result {
                None => break,
                Some(ref r) => match self.current_token() {
                    None => break,
                    Some(token) => match token {
                        Token::Add => {
                            self.next();
                            let right_node = self.factor().unwrap();
                            result = Some(Rc::new(Box::new(ast::Add(r.clone(), right_node))))
                        }
                        Token::Subtract => {
                            self.next();
                            let right_node = self.factor().unwrap();
                            result = Some(Rc::new(Box::new(ast::Subtract(r.clone(), right_node))));
                        }
                        _ => break,
                    },
                },
            }
        }

        return result.unwrap();
    }

    pub fn factor(&mut self) -> Option<ast::Node> {
        let mut factor = self.term();

        loop {
            match factor {
                None => break,
                Some(ref r) => match self.current_token() {
                    None => break,
                    Some(token) => match token {
                        Token::Multiply => {
                            self.next();
                            let right_node = self.factor().unwrap();
                            factor = Some(Rc::new(Box::new(ast::Multiply(r.clone(), right_node))));
                        }
                        Token::Divide => {
                            self.next();
                            let right_node = self.factor().unwrap();
                            factor = Some(Rc::new(Box::new(ast::Divide(r.clone(), right_node))));
                        }
                        _ => break,
                    },
                },
            }
        }

        return factor;
    }

    pub fn term(&mut self) -> Option<ast::Node> {
        let mut term = None;

        match self.current_token().unwrap() {
            Token::Lbrace => {
                self.next();
                term = Some(self.parse_expr());
                if *self.current_token().unwrap() != Token::Rbrace {
                    panic!("Missing rbrace");
                }
            }
            Token::Number(val) => term = Some(Rc::new(Box::new(ast::Leaf(*val)))),
            _ => {}
        }

        return term;
    }
}