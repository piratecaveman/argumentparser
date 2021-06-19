#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    Flag,
    Word,
    Vector,
}

/// from &str  
/// accepted values are: "flag" "word" and "vector"
impl From<&str> for ArgumentType {
    fn from(s: &str) -> ArgumentType {
        match s {
            "flag" => ArgumentType::Flag,
            "word" => ArgumentType::Word,
            "vector" => ArgumentType::Vector,
            _ => panic!("{} :type not found", s),
        }
    }
}

/// from String  
/// accepted values are: "flag" "word" and "vector"
impl From<String> for ArgumentType {
    fn from(s: String) -> ArgumentType {
        match s.as_str() {
            "flag" => ArgumentType::Flag,
            "word" => ArgumentType::Word,
            "vector" => ArgumentType::Vector,
            _ => panic!("{} :type not found", s),
        }
    }
}

/// from &String  
/// accepted values are: "flag" "word" and "vector"
impl From<&String> for ArgumentType {
    fn from(s: &String) -> ArgumentType {
        match s.as_str() {
            "flag" => ArgumentType::Flag,
            "word" => ArgumentType::Word,
            "vector" => ArgumentType::Vector,
            _ => panic!("{} :type not found", s),
        }
    }
}

/// String from ArgumentTypes
impl From<ArgumentType> for String {
    fn from(s: ArgumentType) -> String {
        match s {
            ArgumentType::Flag => "flag",
            ArgumentType::Word => "word",
            ArgumentType::Vector => "vector",
        }
        .to_string()
    }
}

impl AsRef<str> for ArgumentType {
    fn as_ref(&self) -> &str {
        match self {
            ArgumentType::Flag => "flag",
            ArgumentType::Word => "word",
            ArgumentType::Vector => "vector",
        }
    }
}

impl std::fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// default is "flag" type
impl Default for ArgumentType {
    fn default() -> ArgumentType {
        ArgumentType::Flag
    }
}
