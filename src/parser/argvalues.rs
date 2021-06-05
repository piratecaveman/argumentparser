#[derive(Debug, Clone)]
pub enum ArgValue {
    Flag(bool),
    Text(String),
    Vector(Vec<String>),
}

impl Default for ArgValue {
    fn default() -> ArgValue {
        ArgValue::Flag(true)
    }
}

impl ArgValue {
    pub fn arg_type(&self) -> String {
        match self {
            ArgValue::Flag(_) => "flag",
            ArgValue::Text(_) => "text",
            ArgValue::Vector(_) => "vector",
        }
        .to_string()
    }
}
