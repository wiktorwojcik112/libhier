use crate::report;

use crate::expression::Expression;
use crate::interpolated_string::InterpolatedString;
use crate::location::Location;
use crate::token::Token;
use crate::value::Value;

pub struct Parser {
    pub code: Expression,
    current_index: usize,
    tokens: Vec<Token>,
    had_error: bool,
    pub module_reader: fn(String) -> String,
    pub exit_handler: fn() -> !
}

impl Parser {
    pub fn new(tokens: Vec<Token>, module_reader: fn(String) -> String, exit_handler: fn() -> !) -> Self {
        Self {
            code: Expression::NUMBER(0.0),
            current_index: 0,
            tokens,
            had_error: false,
            module_reader,
            exit_handler
        }
    }

    /// Returns bool if there was a error.
    pub fn parse(&mut self) -> bool {
        self.code = self.parse_list()[0].clone();

        self.had_error
    }

    pub fn parse_list(&mut self) -> Vec<Expression> {
        let mut current_list: Vec<Expression> = vec![];

        while self.current_index < self.tokens.len() {
            let current_token = self.consume().clone();

            match current_token {
                Token::LEFT_BRACKET(_) => current_list.push(Expression::LIST(self.parse_list())),
                Token::RIGHT_BRACKET(_) => return current_list,
                Token::LEFT_CURLY(_) => current_list.push(Expression::BLOCK(self.parse_block())),
                Token::RIGHT_CURLY(_) => report("Unexpected }.", (*current_token.get_location()).clone()),
                Token::STRING(string, _) => current_list.push(Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler))),
                Token::NUMBER(number, _) => current_list.push(Expression::NUMBER(number.clone())),
                Token::IDENTIFIER(identifier, _) => {
                    let result = self.parse_identifier(identifier, &mut current_list, true);
                    current_list.push(result);
                },
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();
                        if let Token::IDENTIFIER(identifier, _) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier.to_string()));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();

                        let mut key_expression = Expression::NUMBER(0.0);
                        if let Token::LEFT_CURLY(_) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block());
                        } else if let Token::LEFT_BRACKET(_) = current_token {
                            key_expression = Expression::LIST(self.parse_list());
                        } else if let Token::STRING(string, _) = current_token {
                            key_expression = Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler));
                        } else if let Token::NUMBER(number, _) = current_token {
                            key_expression = Expression::NUMBER(number.clone());
                        } else if let Token::IDENTIFIER(identifier, _) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string()), last_expression, key_expression]))
                    } else {
                        report("Subscript must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::RIGHT_SQUARE(_) => report("Unexpected ].", (*current_token.get_location()).clone()),
                Token::COLON(_) => report("Unexpected :.", (*current_token.get_location()).clone()),
            }
        }

        current_list
    }

    pub fn parse_block(&mut self) -> Vec<Expression> {
        let mut current_list: Vec<Expression> = vec![];

        while self.current_index < self.tokens.len() {
            let current_token = self.consume().clone();

            match current_token {
                Token::LEFT_BRACKET(_) => current_list.push(Expression::LIST(self.parse_list())),
                Token::RIGHT_BRACKET(_) => report("Unexpected ).", (*current_token.get_location()).clone()),
                Token::LEFT_CURLY(_) => current_list.push(Expression::BLOCK(self.parse_block())),
                Token::RIGHT_CURLY(_) => return current_list,
                Token::STRING(string, _) => current_list.push(Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler))),
                Token::NUMBER(number, _) => current_list.push(Expression::NUMBER(number.clone())),
                Token::IDENTIFIER(identifier, _) => {
                    let result = self.parse_identifier(identifier, &mut current_list, false);
                    current_list.push(result);
                },
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        self.consume();

                        let current_token = Token::DOT(Location::new("".to_string(),0, 0));

                        if let Token::IDENTIFIER(identifier, _) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();

                        let mut key_expression = Expression::NUMBER(0.0);
                        if let Token::LEFT_CURLY(_) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block());
                        } else if let Token::LEFT_BRACKET(_) = current_token {
                            key_expression = Expression::LIST(self.parse_list());
                        } else if let Token::STRING(string, _) = current_token {
                            key_expression = Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler));
                        } else if let Token::NUMBER(number, _) = current_token {
                            key_expression = Expression::NUMBER(number.clone());
                        } else if let Token::IDENTIFIER(identifier, _) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string()), last_expression, key_expression]))
                    } else {
                        report("Subscript must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::RIGHT_SQUARE(_) => report("Unexpected ].", (*current_token.get_location()).clone()),
                Token::COLON(_) => report("Unexpected :.", (*current_token.get_location()).clone()),
            }
        }

        current_list
    }

    pub fn parse_expression(&mut self) -> Expression {
        let current_token = self.consume().clone();

        let expression = match current_token {
            Token::LEFT_BRACKET(_) => Expression::LIST(self.parse_list()),
            Token::RIGHT_BRACKET(_) => { report("Unexpected ).", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_CURLY(_) => Expression::BLOCK(self.parse_block()),
            Token::RIGHT_CURLY(_) => { report("Unexpected }.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::STRING(string, _) => Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler)),
            Token::NUMBER(number, _) => Expression::NUMBER(number.clone()),
            Token::IDENTIFIER(identifier, _) => self.parse_identifier(identifier, &mut vec![], false),
            Token::DOT(_) => { report("Unexpected ..", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_SQUARE(_) => { report("Unexpected [.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::RIGHT_SQUARE(_) => { report("Unexpected ].", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::COLON(_) => { report("Unexpected :.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
        };

        expression
    }

    fn parse_identifier(&mut self, identifier: String, current_list: &mut Vec<Expression>, is_list: bool) -> Expression {
        if identifier == ">" {
            /*
            Parse piping. Pipe is represented using > symbol. When pipe is used, previous list is placed inside next list as first argument.
            This allows for more readable chaining of long commands. For example, instead of
            (print (map (1 2 3) { (+ element 1 }))
            you can write
            (1 2 3) > (map { (+ element 1) }) > (print)
            The piping syntax is converted into the first example so it has the same effect.
            */

            if current_list.len() == 0 && is_list {
                // Current list is empty when the identifier is the first element of the list, which means that it should be a name for function > (more than).
                return Expression::IDENTIFIER(identifier.clone().to_string());
            } else if current_list.len() == 0 && !is_list {
                report("Unexpected pipe operator (>). It should be placed after a list.", Location::empty());
                return Expression::VALUE(Value::NULL);
            }

            let last_expression = current_list[current_list.len() - 1].clone();
            current_list.remove(current_list.len() - 1);

            let next_token = self.consume();

            match next_token {
                Token::LEFT_BRACKET(_) => { }
                _ => report("There must be a list after the pipe operator (>).", next_token.get_location().clone())
            }

            let mut next_expression = self.parse_list();

            next_expression.insert(1, last_expression);

            Expression::LIST(next_expression)
        } else {
            if let Token::COLON(_) = self.peek().clone() {
                self.consume();
                let value = self.parse_expression();
                Expression::KEY_VALUE(identifier.to_string().clone(), Box::new(value))
            } else {
                Expression::IDENTIFIER(identifier.clone().to_string())
            }
        }
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.current_index];
        self.current_index += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_index]
    }
}