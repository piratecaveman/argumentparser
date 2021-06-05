use crate::parser::argvalues::ArgValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedArguments {
    pub args: HashMap<String, ArgValue>,
}

impl Default for ParsedArguments {
    fn default() -> ParsedArguments {
        ParsedArguments {
            args: HashMap::new(),
        }
    }
}

impl ParsedArguments {
    pub fn new() -> ParsedArguments {
        ParsedArguments::default()
    }
}
