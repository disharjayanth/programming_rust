use thiserror::Error;
// use std::fmt;

// #[derive(Debug, Clone)]
#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// impl fmt::Display for JsonError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(f, "{} ({} : {})", self.message, self.line, self.column)
//     }
// }

// impl std::error::Error for JsonError {}

fn print_json_error() -> Result<String, JsonError> {
    return Err(JsonError {
        message: "expected ']' at end of array".to_string(),
        line: 10,
        column: 22,
    });
}

fn main() {
    match print_json_error() {
        Ok(some_string) => {
            println!("{}", some_string);
        }
        Err(err) => {
            println!("error while calling PrintJsonError: {}", err);
        }
    }
}
