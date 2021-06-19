#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Flag(bool),
    Word(String),
    Vector(Vec<String>),
}

impl Default for Value {
    fn default() -> Value {
        Value::Flag(true)
    }
}

impl From<bool> for Value {
    fn from(s: bool) -> Value {
        Value::Flag(s)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Word(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::Word(s.to_string())
    }
}

impl From<Vec<String>> for Value {
    fn from(s: Vec<String>) -> Value {
        Value::Vector(s)
    }
}

impl From<&Vec<String>> for Value {
    fn from(s: &Vec<String>) -> Value {
        Value::Vector(s.clone())
    }
}

impl From<Vec<&str>> for Value {
    fn from(s: Vec<&str>) -> Value {
        let new_s = s.into_iter().map(str::to_string).collect::<Vec<String>>();
        Value::Vector(new_s)
    }
}

impl From<&[String]> for Value {
    fn from(s: &[String]) -> Value {
        let new_s = s.to_vec();
        Value::Vector(new_s)
    }
}

impl From<&[&str]> for Value {
    fn from(s: &[&str]) -> Value {
        let new_s = s
            .to_vec()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        Value::Vector(new_s)
    }
}

mod values_tests {
    #[test]
    fn from_bool() {
        let truthful = super::Value::from(false);
        assert!(matches!(truthful, super::Value::Flag(false)));
    }
    #[test]
    fn from_str() {
        let pi = super::Value::from("3.14");
        assert!(matches!(pi, super::Value::Word(_)));
    }
    #[test]
    fn from_string() {
        let radio = super::Value::from(String::from("radio"));
        assert!(matches!(radio, super::Value::Word(_)));
    }
    #[test]
    fn from_vector_string() {
        let picnic_items = super::Value::from(vec![
            "towel".to_string(),
            "fruits".to_string(),
            "screwdriver".to_string(),
        ]);
        assert!(matches!(picnic_items, super::Value::Vector(_)));
    }
    #[test]
    fn from_vector_str() {
        let colors = super::Value::from(vec!["red", "blue", "green"]);
        assert!(matches!(colors, super::Value::Vector(_)));
    }
    #[test]
    fn from_slice_string() {
        let fruits = [
            String::from("oranges"),
            String::from("apples"),
            String::from("Tomatoes"),
        ];
        let fruit_value = super::Value::from(&fruits[1..]);
        assert!(matches!(fruit_value, super::Value::Vector(_)));
    }
    #[test]
    fn from_slice_str() {
        let tv_shows = ["mista robot", "jack ryan", "flapjack bryan"];
        let tv_shows_value = super::Value::from(&tv_shows[1..]);
        assert!(matches!(tv_shows_value, super::Value::Vector(_)));
    }
}
