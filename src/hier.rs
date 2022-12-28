use std::{io, panic};
use std::io::Write;
use crate::environment::{Environment, VariableId};
use crate::expression::Expression;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use crate::value::Value;

pub struct Hier {
    environment: Environment,
    module_reader: fn(String) -> String,
    exit_handler: fn() -> !
}

impl Hier {
    pub fn new(module_reader: fn(String) -> String, exit_handler: fn() -> !) -> Self {
        Self {
            environment: Environment::new(false, module_reader, exit_handler),
            module_reader,
            exit_handler
        }
    }

    pub fn run(&mut self, code: String) -> Value {
        let mut tokenizer = Tokenizer::new(code, self.module_reader, self.exit_handler);

        if tokenizer.tokenize() {
            eprintln!("Failed.");
            (self.exit_handler)();
        }

        let mut parser = Parser::new(tokenizer.tokens);

        if parser.parse() {
            eprintln!("Failed.");
            (self.exit_handler)();
        }

        self.environment.code = parser.code;
        self.environment.interpret()
    }

    pub fn add_function(&mut self, name: String, arguments_count: usize, function: fn(&mut Environment, Vec<Value>) -> Value) {
        self.environment.values.insert(VariableId(0, name), Value::NATIVE_FUNCTION(function, arguments_count));
    }

    pub fn add_variable(&mut self, name: String, value: Value) {
        self.environment.values.insert(VariableId(0, name), value);
    }

    pub fn repl(&mut self) -> ! {
        let mut repl_environment = Environment::new(true, self.module_reader, self.exit_handler);

        loop {
            print!("> ");
            std::io::stdout().flush().expect("Failed to flush stdout.");

            let mut line = String::new();
            if let Err(error) = io::stdin().read_line(&mut line) {
                eprintln!("Failed to read line: {}.", error);
                (self.exit_handler)();
            };

            if line == "(exit)\n" {
                (self.exit_handler)();
            }

            let mut tokenizer = Tokenizer::new(line, self.module_reader, self.exit_handler);

            tokenizer.module_name = "REPL".to_string();

            if tokenizer.tokenize() {
                eprintln!("Failed.");
                continue;
            }

            let mut parser = Parser::new(tokenizer.tokens);

            if parser.parse() {
                eprintln!("Failed.");
                continue;
            }

            let code = if let Expression::BLOCK(code) = parser.code {
                code
            } else {
                vec![parser.code]
            };

            let environment = repl_environment.clone();

            let current_hook = panic::take_hook();

            panic::set_hook(Box::new(|_info| {
                // Do nothing.
            }));

            let value = panic::catch_unwind(move || {
                let mut environment = environment.clone();
                let value = environment.interpret_block(code);
                (value, environment.values)
            });

            panic::set_hook(current_hook);

            match value {
                Ok((value, values)) => { println!("{}", value.text_representation()); repl_environment.values = values },
                _ => { }
            }
        }
    }
}