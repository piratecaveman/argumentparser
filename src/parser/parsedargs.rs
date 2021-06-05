use crate::parser::argvalues::ArgValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ParsedArguments {
    pub args: HashMap<String, ArgValue>,
}

impl ParsedArguments {
    pub fn new() -> ParsedArguments {
        ParsedArguments::default()
    }
}
