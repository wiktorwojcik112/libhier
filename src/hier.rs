use crate::environment::{Environment, VariableId};
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
        let mut code = code;

        if !code.starts_with('(') {
            code.insert(0, '(');
            code.push(')');
        }

        let mut tokenizer = Tokenizer::new(code);

        if tokenizer.tokenize_module() {
            println!("Failed.");
            (self.exit_handler)();
        }

        let mut parser = Parser::new(tokenizer.tokens, self.module_reader, self.exit_handler);

        if parser.parse() {
            println!("Failed.");
            (self.exit_handler)();
        }

        self.environment.code = parser.code;
        self.environment.interpret()
    }

    pub fn add_function(&mut self, name: String, arguments_count: i64, function: fn(&mut Environment, Vec<Value>) -> Value) {
        if arguments_count < -1 {
            panic!("Invalid argument count for function {}. Must be either -1 (infinite) or 0 and higher.", name);
        }

        self.environment.values.insert(VariableId(0, name), Value::NATIVE_FUNCTION(function, arguments_count));
    }

    pub fn add_variable(&mut self, name: String, value: Value) {
        self.environment.values.insert(VariableId(0, name), value);
    }
}