use std::{io::{Error, ErrorKind}, rc::Rc};

use crate::{scanner::{Token, TokenType}, ast::{Expr, Value, Stmt}, error};

pub struct Parser {
    pub tokens: Vec<Rc<Token>>,
    pub current: usize
}

impl Parser {
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            let statement = match self.statement() { // TODO: declaration()
                Ok(statement) => statement,
                Err(e) => { 
                    println!("Error occured while parsing: {}", e);
                    self.synchronise();
                    continue; 
            } 
            };
            statements.push(statement)
        }

        statements
    }

    // fn declaration(&mut self) -> Result<Stmt, Error> {
    //     match self.peek().typ {
    //         TokenType::Let => {
    //             self.advance();
    //             self.var_declaration()
    //         },
    //         TokenType::Fn => {
    //             self.advance();
    //             self.function()
    //         }
    //         _ => self.statement()
    //     }
    // }

    // fn function(&mut self) -> Result<Stmt, Error> {
    //     let name = Rc::clone(self.consume(TokenType::Identifier, "Expect function name.")?);
    //     self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
    //     let mut params = vec![];
    //     match self.peek().typ {
    //         TokenType::RightParen => (),
    //         _ => {
    //             loop {
    //                 params.push(Rc::clone(self.consume(TokenType::Identifier, "Expect parameter name.")?));

    //                 match self.peek().typ {
    //                     TokenType::Comma => {
    //                         self.advance();
    //                     },
    //                     _ => break
    //                 }
    //             }
    //         }
    //     }
    //     self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

    //     self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
    //     let body = self.block()?;

    //     Ok(Stmt::Fun { name, params, body: Rc::new(Stmt::Block { statements: body }) })
    // }

    // fn var_declaration(&mut self) -> Result<Stmt, Error> {
    //     let name = Rc::clone(self.consume(TokenType::Identifier, "Expect variable name.")?);

    //     let initializer = match self.peek().typ {
    //         TokenType::Equal => {
    //             self.advance();
    //             self.expression()
    //         }
    //         _ => Ok(Expr::Literal { value: Value::Null })
    //     };

    //     if let Err(e) = initializer {
    //         return Err(e);
    //     }

    //     self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
    //     Ok(Stmt::Let { name, initializer: Box::new(initializer.unwrap()) })
    // }

    fn statement(&mut self) -> Result<Stmt, Error> {
        match self.peek().typ {
            TokenType::Print => {
                self.advance();    
                self.print_statement()
            },
            TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::Block { statements: self.block()? })
            },
            TokenType::If => {
                self.advance();
                self.if_statement()
            },
            TokenType::While => {
                self.advance();
                self.while_statement()
            },
            TokenType::Faran => {
                self.advance();
                self.faran_statement()
            },
            TokenType::Ke => {
                self.advance();
                self.ke_statement()
            }
            // TokenType::For => {
            //     self.advance();
            //     self.for_statement()
            // },
            // TokenType::Return => {
            //     self.advance();
            //     self.return_statement()
            // }
            _ => self.expression_statement()
        }
    }

    // fn return_statement(&mut self) -> Result<Stmt, Error> {
    //     let keyword = Rc::clone(self.previous());

    //     let expr = match self.peek().typ {
    //         TokenType::Semicolon => {
    //             Expr::Literal { value: Value::Null }
    //         },
    //         _ => self.expression()?
    //     };

    //     self.consume(TokenType::Semicolon, "Expect ';' after return statement.")?;
    //     Ok(Stmt::Return { keyword, value: Box::new(expr) })
    // }

    // fn for_statement(&mut self) -> Result<Stmt, Error> {
    //     self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
    //     let initializer = match self.peek().typ {
    //         TokenType::Semicolon => {
    //             self.advance();
    //             None
    //         },
    //         TokenType::Let => {
    //             self.advance();
    //             Some(self.var_declaration()?)
    //         },
    //         _ => {
    //             Some(self.expression_statement()?)
    //         }
    //     };
    //     let mut condition = match self.peek().typ {
    //         TokenType::Semicolon => {
    //             None
    //         },
    //         _ => {
    //             Some(self.expression()?)
    //         }
    //     };
    //     self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
    //     let increment = match self.peek().typ {
    //         TokenType::RightParen => {
    //             None
    //         },
    //         _ => {
    //             Some(self.expression()?)
    //         }
    //     };
    //     self.consume(TokenType::RightParen, "Expect ')' after for loop.")?;
        
    //     let mut body = self.statement()?;
    //     if let Some(inc) = increment {
    //         body = Stmt::Block { statements: vec![body, Stmt::Expression { expression: Box::new(inc) }] };
    //     }
    //     if condition.is_none() {
    //         condition = Some(Expr::Literal { value: Value::Boolean(true) });
    //     }
    //     body = Stmt::While { condition: Box::new(condition.unwrap()), body: Box::new(body) };
    //     if let Some(init) = initializer {
    //         body = Stmt::Block { statements: vec![init, body] };
    //     }

    //     Ok(body)
    // }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression().expect("expression expected");
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = self.statement().expect("statement expected");
        Ok(Stmt::While { condition: Box::new(condition), body: Box::new(body) })
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression().expect("expression expected");
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let then = self.statement().expect("statement expected");
        match self.peek().typ {
            TokenType::Else => {
                self.advance();
                let els = self.statement().expect("statement expected");
                Ok(Stmt::If { condition: Box::new(condition), then: Box::new(then), els: Some(Box::new(els)) })
        },
            _ => Ok(Stmt::If { condition: Box::new(condition), then: Box::new(then), els: None })
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression().expect("expression expected");
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: Box::new(value) })
    }

    fn faran_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Faran)
    }

    fn ke_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Ke)
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = vec![];

        while !match self.peek().typ {
            TokenType::RightBrace => true,
            _ => false
        } && !self.is_at_end() {
            let stmt = match self.statement() { // TODO: declaration()
                Ok(stmt) => stmt,
                Err(e) => return Err(e)
            };
            statements.push(stmt);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression().expect("expression expected");
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression { expression: Box::new(expr) })
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    // fn assignement(&mut self) -> Result<Expr, Error> {
    //     let expr = self.or();

    //     if match self.peek().typ {
    //         TokenType::Equal => {
    //             self.advance();
    //             true
    //         },
    //         _ => false
    //     } {
    //         let equals = Rc::clone(self.previous());
    //         let value = self.assignement();

    //         if let Err(e) = value {
    //             return Err(e);
    //         }

    //         return match expr {
    //             Ok(Expr::Variable { ref name }) => Ok(Expr::Assign { name: Rc::clone(name), value: Box::new(value.unwrap()) }),
    //             _ => Err(self.error(&equals, "Invalid assignement target."))
    //         };
    //     }

    //     expr
    // }

    // fn or(&mut self) -> Result<Expr, Error> {
    //     let expr = self.and()?;

    //     while match self.peek().typ {
    //         TokenType::Or => {
    //             self.advance();
    //             true
    //         },
    //         _ => false
    //     } {
    //         let operator = Rc::clone(self.previous());
    //         let right = self.and()?;
    //         return Ok(Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) });
    //     }

    //     Ok(expr)
    // }

    // fn and(&mut self) -> Result<Expr, Error> {
    //     let expr = self.equality()?;

    //     while match self.peek().typ {
    //         TokenType::And => {
    //             self.advance();
    //             true
    //         },
    //         _ => false
    //     } {
    //         let operator = Rc::clone(self.previous());
    //         let right = self.equality()?;
    //         return Ok(Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) });
    //     }

    //     Ok(expr)
    // }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while match self.peek().typ {
            TokenType::BangEqual | TokenType::EqualEqual => {
                self.advance();
                true
            },
            _ => false
        } {
            let operator = Rc::clone(self.previous());
            let right = match self.comparison() {
                Ok(right) => right,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while match self.peek().typ {
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                self.advance();
                true
            },
            _ => false
        } {
            let operator = Rc::clone(self.previous());
            let right = match self.term() {
                Ok(right) => right,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while match self.peek().typ {
            TokenType::Minus | TokenType::Plus => {
                self.advance();
                true
            },
            _ => false
        } {
            let operator = Rc::clone(self.previous());
            let right = match self.factor() {
                Ok(right) => right,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while match self.peek().typ {
            TokenType::Star | TokenType::Slash => {
                self.advance();
                true
            },
            _ => false
        } {
            let operator = Rc::clone(self.previous());
            let right = match self.unary() {
                Ok(right) => right,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if match self.peek().typ {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                true
            },
            _ => false
        } {
            let operator = Rc::clone(self.previous());
            let right = match self.unary() {
                Ok(right) => right,
                Err(e) => return Err(e)
            };
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }

        // self.call()
        self.primary()
    }

    // fn call(&mut self) -> Result<Expr, Error> {
    //     let mut expr = self.primary()?;

    //     loop {
    //         match self.peek().typ {
    //             TokenType::LeftParen => {
    //                 self.advance();
    //                 expr = self.finish_call(expr)?;
    //             },
    //             _ => break
    //         }
    //     }

    //     Ok(expr)
    // }

    // fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
    //     let mut arguments: Vec<Box<Expr>> = vec![];
    //     match self.peek().typ {
    //         TokenType::RightParen => (),
    //         _ => {
    //             loop {
    //                 arguments.push(Box::new(self.expression()?));
    //                 match self.peek().typ {
    //                     TokenType::Comma => {
    //                         self.advance();
    //                     },
    //                     _ => {
    //                         break;
    //                     }
    //                 };
    //             }
    //         }
    //     }

    //     let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

    //     Ok(Expr::Call { callee: Box::new(callee), paren: Rc::clone(paren), arguments })
    // }

    fn primary(&mut self) -> Result<Expr, Error> {
        if let Ok(res) = match &self.peek().typ {
            TokenType::False => Ok(Expr::Literal { value: Value::Boolean(false) }),
            TokenType::True => Ok(Expr::Literal { value: Value::Boolean(true) }),
            TokenType::Null => Ok(Expr::Literal { value: Value::Null }),
            TokenType::Number(n) => Ok(Expr::Literal { value: Value::Number(*n) }),
            TokenType::String(s) => Ok(Expr::Literal { value: Value::String(s.clone()) }),
            TokenType::Soro => Ok(Expr::Soro),
            // TokenType::Identifier => Ok(Expr::Variable { name: Rc::clone(self.peek()) }),
            TokenType::LeftParen => {
                self.advance();
                let expr = match self.expression() {
                    Ok(right) => right,
                    Err(e) => return Err(e)
                };
                match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                    Ok(_) => {
                        self.current -= 1;
                        Ok(Expr::Grouping { expression: Box::new(expr) })
                    },
                    Err(_) => Err(())
                }
            },
            _ => Err(())
        } {
            self.advance();
            return Ok(res);
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<&Rc<Token>, Error> {
        if self.check(typ) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &str) -> Error {
        Error::new(ErrorKind::Other, error(token.line, message))
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if matches!(self.previous().typ, TokenType::Semicolon) {
                return;
            }

            match self.peek().typ {
                TokenType::Fn | TokenType::Let | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    return;
                }
                _ => ()
            }

            self.advance();
        }
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().typ == typ
    }

    fn advance(&mut self) -> &Rc<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().typ, TokenType::EOF)
    }

    fn peek(&self) -> &Rc<Token> {
        self.tokens.get(self.current).expect("token expected")
    }

    fn previous(&self) -> &Rc<Token> {
        self.tokens.get(self.current - 1).expect("token expected")
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{scanner::{Token, TokenType}, ast::{Expr, Value}};

    use super::Parser;

    #[test]
    fn test_parse_primary() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "\"string\"".into(), line: 0, typ: TokenType::String("string".into()) }),
            Rc::new(Token { lexeme: "true".into(), line: 0, typ: TokenType::True }),
            Rc::new(Token { lexeme: "false".into(), line: 0, typ: TokenType::False }),
            Rc::new(Token { lexeme: "fu".into(), line: 0, typ: TokenType::Null }),
            Rc::new(Token { lexeme: "(".into(), line: 0, typ: TokenType::LeftParen }),
            Rc::new(Token { lexeme: "true".into(), line: 0, typ: TokenType::True }),
            Rc::new(Token { lexeme: ")".into(), line: 0, typ: TokenType::RightParen }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Literal { value: Value::Number(12.0) },
            Expr::Literal { value: Value::String("string".into()) },
            Expr::Literal { value: Value::Boolean(true) },
            Expr::Literal { value: Value::Boolean(false) },
            Expr::Literal { value: Value::Null },
            Expr::Grouping { expression: Box::new(Expr::Literal { value: Value::Boolean(true) }) }
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.primary().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_unary() {
        let tokens = vec![
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "!".into(), line: 0, typ: TokenType::Bang }),
            Rc::new(Token { lexeme: "false".into(), line: 0, typ: TokenType::False }),
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "!".into(), line: 0, typ: TokenType::Bang }),
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "true".into(), line: 0, typ: TokenType::True }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Unary { operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), right: Box::new(Expr::Literal { value: Value::Number(12.0) }) },
            Expr::Unary { operator: Rc::new(Token { lexeme: "!".into(), line: 0, typ: TokenType::Bang }), right: Box::new(Expr::Literal { value: Value::Boolean(false) }) },
            Expr::Unary { 
                operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), 
                right: Box::new(Expr::Unary { 
                    operator: Rc::new(Token { lexeme: "!".into(), line: 0, typ: TokenType::Bang }), 
                    right: Box::new(Expr::Unary { 
                        operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), 
                        right: Box::new(Expr::Literal { value: Value::Boolean(true) })
                    })
                })
            }
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.unary().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_factor() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }),
            Rc::new(Token { lexeme: "0.1".into(), line: 0, typ: TokenType::Number(0.1) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "/".into(), line: 0, typ: TokenType::Slash }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "/".into(), line: 0, typ: TokenType::Slash }),
            Rc::new(Token { lexeme: "4".into(), line: 0, typ: TokenType::Number(4.0) }),
            Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }), 
                right: Box::new(Expr::Literal { value: Value::Number(0.1) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "/".into(), line: 0, typ: TokenType::Slash }), 
                right: Box::new(Expr::Literal { value: Value::Number(12.0) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Binary { 
                    left: Box::new(Expr::Binary { 
                        left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                        operator: Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }), 
                        right: Box::new(Expr::Literal { value: Value::Number(2.0) })  
                    }), 
                    operator: Rc::new(Token { lexeme: "/".into(), line: 0, typ: TokenType::Slash }), 
                    right: Box::new(Expr::Literal { value: Value::Number(4.0) }) 
                }), 
                operator: Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }), 
                right: Box::new(Expr::Literal { value: Value::Number(2.0) }) 
            },
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.factor().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_term() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }),
            Rc::new(Token { lexeme: "0.1".into(), line: 0, typ: TokenType::Number(0.1) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "4".into(), line: 0, typ: TokenType::Number(4.0) }),
            Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }), 
                right: Box::new(Expr::Literal { value: Value::Number(0.1) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), 
                right: Box::new(Expr::Literal { value: Value::Number(12.0) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Binary { 
                    left: Box::new(Expr::Binary { 
                        left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                        operator: Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }), 
                        right: Box::new(Expr::Literal { value: Value::Number(2.0) })  
                    }), 
                    operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), 
                    right: Box::new(Expr::Literal { value: Value::Number(4.0) }) 
                }), 
                operator: Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }), 
                right: Box::new(Expr::Literal { value: Value::Number(2.0) }) 
            },
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.term().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_comparison() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }),
            Rc::new(Token { lexeme: "0.1".into(), line: 0, typ: TokenType::Number(0.1) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: ">".into(), line: 0, typ: TokenType::Greater }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: ">=".into(), line: 0, typ: TokenType::GreaterEqual }),
            Rc::new(Token { lexeme: "4".into(), line: 0, typ: TokenType::Number(4.0) }),
            Rc::new(Token { lexeme: "<=".into(), line: 0, typ: TokenType::LessEqual }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }), 
                right: Box::new(Expr::Literal { value: Value::Number(0.1) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: ">".into(), line: 0, typ: TokenType::Greater }), 
                right: Box::new(Expr::Literal { value: Value::Number(12.0) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Binary { 
                    left: Box::new(Expr::Binary { 
                        left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                        operator: Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }), 
                        right: Box::new(Expr::Literal { value: Value::Number(2.0) })  
                    }), 
                    operator: Rc::new(Token { lexeme: ">=".into(), line: 0, typ: TokenType::GreaterEqual }), 
                    right: Box::new(Expr::Literal { value: Value::Number(4.0) }) 
                }), 
                operator: Rc::new(Token { lexeme: "<=".into(), line: 0, typ: TokenType::LessEqual }), 
                right: Box::new(Expr::Literal { value: Value::Number(2.0) }) 
            },
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.comparison().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_equality() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }),
            Rc::new(Token { lexeme: "0.1".into(), line: 0, typ: TokenType::Number(0.1) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }),
            Rc::new(Token { lexeme: "4".into(), line: 0, typ: TokenType::Number(4.0) }),
            Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }), 
                right: Box::new(Expr::Literal { value: Value::Number(0.1) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                operator: Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }), 
                right: Box::new(Expr::Literal { value: Value::Number(12.0) }) 
            },
            Expr::Binary { 
                left: Box::new(Expr::Binary { 
                    left: Box::new(Expr::Binary { 
                        left: Box::new(Expr::Literal { value: Value::Number(12.0) }), 
                        operator: Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }), 
                        right: Box::new(Expr::Literal { value: Value::Number(2.0) })  
                    }), 
                    operator: Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }), 
                    right: Box::new(Expr::Literal { value: Value::Number(4.0) }) 
                }), 
                operator: Rc::new(Token { lexeme: "!=".into(), line: 0, typ: TokenType::BangEqual }), 
                right: Box::new(Expr::Literal { value: Value::Number(2.0) }) 
            },
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.equality().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    #[test]
    fn test_parse_expression() {
        let tokens = vec![
            Rc::new(Token { lexeme: "12".into(), line: 0, typ: TokenType::Number(12.0) }),
            Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }),
            Rc::new(Token { lexeme: "(".into(), line: 0, typ: TokenType::LeftParen }),
            Rc::new(Token { lexeme: "0.1".into(), line: 0, typ: TokenType::Number(0.1) }),
            Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }),
            Rc::new(Token { lexeme: "5".into(), line: 0, typ: TokenType::Number(5.0) }),
            Rc::new(Token { lexeme: ")".into(), line: 0, typ: TokenType::RightParen }),
            Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }),
            Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }),
            Rc::new(Token { lexeme: "2".into(), line: 0, typ: TokenType::Number(2.0) }),
            Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }),
            Rc::new(Token { lexeme: "true".into(), line: 0, typ: TokenType::True }),
            Rc::new(Token { lexeme: "".into(), line: 0, typ: TokenType::EOF })
        ];
        let expected = vec![
            Expr::Binary { 
                left: Box::new(Expr::Binary { 
                    left: Box::new(Expr::Literal { value: Value::Number(12.0) }),
                    operator: Rc::new(Token { lexeme: "<".into(), line: 0, typ: TokenType::Less }), 
                    right: Box::new(Expr::Binary { 
                        left: Box::new(Expr::Grouping { 
                            expression: Box::new(Expr::Binary { 
                                left: Box::new(Expr::Literal { value: Value::Number(0.1) }), 
                                operator: Rc::new(Token { lexeme: "+".into(), line: 0, typ: TokenType::Plus }), 
                                right: Box::new(Expr::Literal { value: Value::Number(5.0) })
                            }) 
                        }), 
                        operator: Rc::new(Token { lexeme: "*".into(), line: 0, typ: TokenType::Star }), 
                        right: Box::new(Expr::Unary { 
                            operator: Rc::new(Token { lexeme: "-".into(), line: 0, typ: TokenType::Minus }), 
                            right: Box::new(Expr::Literal { value: Value::Number(2.0) }) 
                        })
                    }) 
                }), 
                operator: Rc::new(Token { lexeme: "==".into(), line: 0, typ: TokenType::EqualEqual }), 
                right: Box::new(Expr::Literal { value: Value::Boolean(true) }) 
            },
        ];

        let mut parser = Parser {
            current: 0,
            tokens
        };

        for expect in expected {
            let parsed = parser.expression().expect("Expr expected.");
            if !equal_expr(&expect,&parsed) {
                panic!("{:?} is not equal to {:?}", parsed, expect);
            }
        }
    }

    fn equal_expr(expr1: &Expr, expr2: &Expr) -> bool {
        match (expr1, expr2) {
            (Expr::Literal { value: v1 }, Expr::Literal { value: v2 }) => v1 == v2,
            (Expr::Unary { operator: op1, right: r1 }, Expr::Unary { operator: op2, right: r2 }) => {
                if op1.typ != op2.typ {
                    return false;
                }

                equal_expr(r1, r2)
            },
            (Expr::Grouping { expression: expr1 }, Expr::Grouping { expression: expr2 }) => equal_expr(expr1, expr2),
            (Expr::Binary { left: l1, operator: op1, right: r1 }, Expr::Binary { left: l2, operator: op2, right: r2 }) => {
                if op1.typ != op2.typ {
                    return false;
                }

                equal_expr(l1, l2) && equal_expr(r1, r2)
            }
            (_, _) => false
        }
    }
}