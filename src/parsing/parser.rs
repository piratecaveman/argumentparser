use std::collections::HashMap;
use std::collections::HashSet;

use crate::Argument;
use crate::ParsedArguments;
use crate::Value;

#[derive(Debug, Clone, Default)]
pub struct Parser {
    pub arguments: HashMap<String, Argument>,
    invocators: HashSet<String>,
    count: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser::default()
    }
    pub fn with_capacity(capacity: usize) -> Parser {
        Parser {
            arguments: HashMap::with_capacity(capacity),
            invocators: HashSet::with_capacity(capacity * 4usize),
            count: 0,
        }
    }
    pub fn add_argument(&mut self, argument: Argument) {
        assert!(&argument.is_valid());
        let is_flag = argument.get_type().eq("flag");
        for item in &argument.invoke_with {
            assert!(
                !self.invocators.contains(item),
                "duplicate invocator found: {}",
                &item
            );
            self.invocators.insert(item.to_string());
            if is_flag {
                if item.starts_with("--") {
                    self.invocators
                        .insert(format!("--no-{}", &item.strip_prefix("--").unwrap()));
                } else if item.starts_with('-') {
                    self.invocators
                        .insert(format!("-no-{}", &item.strip_prefix('-').unwrap()));
                } else {
                    self.invocators.insert(format!("no-{}", &item));
                }
            };
        }
        self.arguments.insert(argument.name.clone(), argument);
        self.count += 1;
    }
    fn detect_negation(s: &str) -> bool {
        s.starts_with("--no-") || s.starts_with("-no-") || s.starts_with("no-")
    }
    fn negation_type(s: &str) -> u8 {
        if s.starts_with("--no-") {
            1u8
        } else if s.starts_with("-no-") {
            2u8
        } else if s.starts_with("no-") {
            3u8
        } else {
            0u8
        }
    }
    fn strip_negation(s: &str) -> String {
        let negation_type = Parser::negation_type(s);
        match negation_type {
            1u8 => s.replacen("--no-", "--", 1),
            2u8 => s.replacen("-no-", "-", 1),
            3u8 => s.replacen("no-", "", 1),
            _ => s.to_string(),
        }
    }
    pub fn contains_argument(&self, other: &str) -> bool {
        let other = Parser::strip_negation(other);
        self.invocators.contains(&other)
    }
    pub fn get_argument(&self, other: &str) -> Option<&Argument> {
        let other = Parser::strip_negation(other);
        self.arguments.values().find(|f| f.same_as(&other))
    }
    pub fn count(&self) -> usize {
        self.count
    }
    fn break_apart(word: &str) -> Option<Vec<String>> {
        if word.starts_with('-') && !word.starts_with("--") {
            let vector = word
                .chars()
                .filter(|f| !f.eq(&'-'))
                .map(|f| format!("-{}", f))
                .collect::<Vec<_>>();
            Some(vector)
        } else if word.starts_with("--") {
            None
        } else {
            let vector = word.chars().map(|f| format!("{}", f)).collect::<Vec<_>>();
            Some(vector)
        }
    }
    pub fn parse_arguments<T: ToString>(&self, arguments: &[T]) -> ParsedArguments {
        let mut parsed = ParsedArguments::new();
        let mut env_arguments = arguments.iter().map(T::to_string).peekable();
        while env_arguments.peek().is_some() {
            let word = env_arguments.next().unwrap();
            if self.invocators.contains(&word) {
                let argument = self.get_argument(&word).unwrap();
                let name = argument.name.clone();
                let negation = Parser::detect_negation(&word);
                match argument.get_type() {
                    "flag" => {
                        parsed.arguments.insert(name, Value::from(!negation));
                        continue;
                    }
                    "word" => {
                        let next = match env_arguments.next() {
                            Some(some_string) => some_string,
                            None => panic!("expected a value for {}", &word),
                        };
                        assert!(
                            !self.contains_argument(&next),
                            "expected a value for {} found argument {}",
                            &word,
                            &next,
                        );
                        let broken = match Parser::break_apart(&next) {
                            Some(broken_value) => broken_value,
                            None => {
                                parsed.arguments.insert(name, Value::from(next));
                                continue;
                            }
                        };
                        assert!(
                            !broken.iter().all(|f| { self.contains_argument(&f) }),
                            "expected a value for {} found argument {}",
                            &word,
                            &next,
                        );
                        parsed.arguments.insert(name, Value::from(next));
                        continue;
                    }
                    "vector" => {
                        let mut value = Vec::<String>::new();
                        assert!(
                            env_arguments.peek().is_some(),
                            "expected value(s) for {}",
                            &word
                        );
                        while env_arguments.peek().is_some() {
                            let next = env_arguments.peek().unwrap();
                            let broken = Parser::break_apart(&next);
                            if self.contains_argument(&next) {
                                assert!(!value.is_empty(), "expected value(s) for {}", &word);
                                parsed.arguments.insert(name, Value::from(value));
                                break;
                            } else if broken.is_none() {
                                value.push(env_arguments.next().unwrap());
                                continue;
                            } else {
                                let broken = broken.unwrap();
                                let is_argument = broken.iter().all(|f| self.contains_argument(f));
                                if value.is_empty() {
                                    assert!(
                                        !is_argument,
                                        "expected value(s) for {} found argument {}",
                                        &word, &next
                                    );
                                }
                                if is_argument {
                                    parsed.arguments.insert(name, Value::from(value));
                                    break;
                                } else {
                                    value.push(env_arguments.next().unwrap());
                                    if env_arguments.peek().is_none() {
                                        parsed.arguments.insert(name, Value::from(value));
                                        break;
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                };
            } else {
                let broken = match Parser::break_apart(&word) {
                    Some(value) => value,
                    None => panic!("unrecognized argument found: {}", &word),
                };
                let is_argument = broken.iter().all(|f| self.contains_argument(f));
                if is_argument {
                    assert!(
                        broken
                            .iter()
                            .all(|f| { self.get_argument(f).unwrap().get_type().eq("flag") }),
                        "found non flag argument clubbed in: {}",
                        &word
                    );
                    for item in broken {
                        let this_arg = self.get_argument(&item).unwrap();
                        parsed
                            .arguments
                            .insert(this_arg.name.clone(), Value::from(true));
                    }
                } else {
                    continue;
                }
            }
        }
        parsed
    }
}

mod parser_tests {
    #[test]
    fn adding_argument() {
        let mut parser = super::Parser::with_capacity(1);
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("pizza")
                // redundant as name is automatically added as an invoke option
                .invoke_with("pizza")
                .invoke_with("--pizza")
                .invoke_with("-p")
                .required(false),
        );
        assert!(parser.contains_argument("pizza"));
    }
    #[test]
    #[should_panic]
    fn duplicate_argument_panic() {
        let mut parser = super::Parser::with_capacity(2);
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("pizza")
                .invoke_with("--pizza")
                .invoke_with("-p")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("pineapple")
                .invoke_with("--pineapple")
                // clashes with pizza's invocator
                .invoke_with("-p")
                .required(false),
        );
    }
    #[test]
    fn get_argument() {
        let mut parser = super::Parser::with_capacity(2);
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("pizza")
                .invoke_with("--pizza")
                .invoke_with("-p")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("pineapple")
                .invoke_with("pineapple")
                .invoke_with("--pineapple")
                .invoke_with("-P")
                .required(false),
        );
        let pineapple = parser.get_argument("pineapple");
        assert!(pineapple.is_some());
        assert!(pineapple.unwrap().same_as("--pineapple"));
    }
    #[test]
    fn parsing_test() {
        let mut parser = super::Parser::with_capacity(8);
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("sleep")
                .invoke_with("--sleep")
                .invoke_with("-s")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("word")
                .name("my-name")
                .invoke_with("--my-name")
                .invoke_with("-n"),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("dragon-colors")
                .invoke_with("--dragon-colors")
                .invoke_with("-d")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("pass")
                .invoke_with("--pass")
                .invoke_with("-p"),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("mangle")
                .invoke_with("--mangle")
                .invoke_with("-M"),
        );
        parser.add_argument(super::Argument::with_type("word").name("cheese"));
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("xylo")
                .invoke_with("x"),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("radio")
                .invoke_with("--radio")
                .invoke_with("r"),
        );
        let parsed = parser.parse_arguments(&[
            "-no-s",
            "--my-name",
            "Jondo",
            "-d",
            "red",
            "blue",
            "orange",
            "-Mp",
            "cheese",
            "swiss",
            "rx",
        ]);
        assert!(parsed.get_value("sleep").is_some());
        assert!(parsed
            .get_value("sleep")
            .unwrap()
            .eq(&super::Value::from(false)));
        assert!(parsed.get_value("my-name").is_some());
        assert!(parsed
            .get_value("my-name")
            .unwrap()
            .eq(&super::Value::from("Jondo")));
        assert!(parsed.get_value("dragon-colors").is_some());
        assert!(parsed
            .get_value("dragon-colors")
            .unwrap()
            .eq(&super::Value::from(&["red", "blue", "orange"][..])));
        assert!(parsed.get_value("mangle").is_some());
        assert!(parsed
            .get_value("mangle")
            .unwrap()
            .eq(&super::Value::from(true)));
        assert!(parsed.get_value("pass").is_some());
        assert!(parsed
            .get_value("pass")
            .unwrap()
            .eq(&super::Value::from(true)));
        assert!(parsed.get_value("cheese").is_some());
        assert!(parsed
            .get_value("cheese")
            .unwrap()
            .eq(&super::Value::from("swiss")));
        assert!(parsed.get_value("radio").is_some());
        assert!(parsed
            .get_value("radio")
            .unwrap()
            .eq(&super::Value::from(true)));
        assert!(parsed.get_value("xylo").is_some());
        assert!(parsed
            .get_value("xylo")
            .unwrap()
            .eq(&super::Value::from(true)));
    }
}
