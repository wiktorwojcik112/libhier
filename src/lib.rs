use crate::location::Location;

pub mod hier;
pub mod value;
pub mod types;
pub mod environment;
pub mod tokenizer;
pub mod parser;
pub mod interpreter;
pub mod native_functions;
pub mod token;
pub mod location;
pub mod expression;

pub use crate::interpreter::warning;

fn report(error: &str, location: Location) {
    eprintln!("! [{}:{}] in {}: {}", location.line_number, location.offset, location.module, error);
}