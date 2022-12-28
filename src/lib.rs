use crate::location::Location;

pub mod hier;
pub mod value;

mod tokenizer;
mod parser;
mod interpreter;
mod environment;
mod native_functions;
mod types;
mod token;
mod location;
mod expression;

fn report(error: &str, location: Location) {
    eprintln!("! [{}:{}] in {}: {}", location.line_number, location.offset, location.module, error);
}