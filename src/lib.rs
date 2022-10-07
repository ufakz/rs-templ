use regex::{Captures, Regex};
use std::{collections::HashMap, fmt};

enum Data {
    Number(i32),
    Boolean(bool),
    Text(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Number(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
