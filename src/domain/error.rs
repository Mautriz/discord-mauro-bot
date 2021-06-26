use std::{
    error::Error,
    fmt::{Display, Formatter},
};
#[derive(Debug)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {}
