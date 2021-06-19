use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ParsedArguments {
    pub arguments: HashMap<String, Value>,
    count: usize,
}

impl ParsedArguments {
    pub fn new() -> ParsedArguments {
        ParsedArguments::default()
    }
    pub fn with_capacity(capacity: usize) -> ParsedArguments {
        ParsedArguments {
            arguments: HashMap::with_capacity(capacity),
            count: 0usize,
        }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn get_value(&self, name: &str) -> Option<&Value> {
        self.arguments.get(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.arguments.contains_key(name)
    }
}
