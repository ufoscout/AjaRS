use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct MyError {}

impl Display for MyError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

