#[derive(Debug, Clone, Copy)]
pub enum AType {
    Flag,
    Text,
    Vector,
}

impl Default for AType {
    fn default() -> AType {
        AType::Flag
    }
}

impl std::fmt::Display for AType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Self::Flag => "flag",
            Self::Text => "text",
            Self::Vector => "vector",
        };
        write!(f, "{}", t)
    }
}

impl From<String> for AType {
    fn from(something: String) -> Self {
        match something.as_ref() {
            "flag" => Self::Flag,
            "text" => Self::Text,
            "vector" => Self::Vector,
            _ => panic!("{} is not a recognized type", something),
        }
    }
}

impl From<&str> for AType {
    fn from(something: &str) -> Self {
        match something {
            "flag" => Self::Flag,
            "text" => Self::Text,
            "vector" => Self::Vector,
            _ => panic!("{} is not a recognized type", something),
        }
    }
}

impl From<AType> for String {
    fn from(some_atype: AType) -> Self {
        some_atype.to_string()
    }
}

mod atype_tests {
    #[test]
    fn from_string() {
        let this = super::AType::from(String::from("flag"));
        assert!(matches!(this, super::AType::Flag));
    }
    #[test]
    fn from_str() {
        let this = super::AType::from("text");
        assert!(matches!(this, super::AType::Text));
    }
    #[test]
    fn to_string() {
        let this = super::AType::Flag.to_string();
        let string = "flag".to_string();
        assert!(string.eq(&this));
    }
    #[test]
    fn string_from() {
        let this = String::from(super::AType::Vector);
        let string = "vector".to_string();
        assert!(string.eq(&this));
    }
    #[test]
    #[should_panic]
    fn unrecognized_type_panic() {
        let this = super::AType::from("dragon");
        dbg!(this);
    }
    #[test]
    fn display_works() {
        let this = super::AType::Text;
        println!("{}", this);
    }
}
