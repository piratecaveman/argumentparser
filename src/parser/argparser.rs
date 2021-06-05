use crate::defarg::arg::Argument;
use crate::defarg::atypes::AType;
use crate::parser::argvalues::ArgValue;
use crate::parser::parsedargs::ParsedArguments;

#[derive(Debug, Default)]
pub struct ArgParser {
    pub parser: std::collections::HashMap<String, Argument>,
}

impl ArgParser {
    pub fn new() -> ArgParser {
        ArgParser::default()
    }
    pub fn add_argument(&mut self, arg: Argument) {
        assert!(&arg.is_valid(), "argument is invalid");
        assert!(!self.contains_arg(&arg.name));
        let short = &arg.short;
        let long = &arg.long;
        if short.is_some() {
            assert!(
                !self.contains_arg(short.as_ref().unwrap()),
                "duplicate short agrument found"
            );
        };
        if long.is_some() {
            assert!(
                !self.contains_arg(long.as_ref().unwrap()),
                "duplicate long argument found"
            );
        };
        self.parser.insert(arg.name.clone(), arg);
    }
    pub fn contains_arg<T: ToString>(&self, identity: T) -> bool {
        let identity = identity.to_string();
        self.parser.contains_key(&identity)
            || self.parser.values().any(|f| {
                f.short
                    .as_ref()
                    .map(|f| f.eq(&identity))
                    .unwrap_or_default()
                    || f.long.as_ref().map(|f| f.eq(&identity)).unwrap_or_default()
            })
    }
    fn is_flag<T: ToString>(&self, identity: T) -> bool {
        let found = self.get_arg(identity);
        match found {
            None => false,
            Some(f) => matches!(f.atype, AType::Flag),
        }
    }
    fn is_text<T: ToString>(&self, identity: T) -> bool {
        let found = self.get_arg(identity);
        match found {
            None => false,
            Some(f) => matches!(f.atype, AType::Text),
        }
    }
    fn get_arg<T: ToString>(&self, identity: T) -> Option<&Argument> {
        let identity = identity.to_string();
        if self.parser.contains_key(&identity) {
            self.parser.get(&identity)
        } else {
            self.parser.values().find(|f| {
                f.short
                    .as_ref()
                    .map(|f| f.eq(&identity))
                    .unwrap_or_default()
                    || f.long.as_ref().map(|f| f.eq(&identity)).unwrap_or_default()
            })
        }
    }
    pub fn parse_args<T: ToString>(&mut self, some_args: &[T]) -> ParsedArguments {
        let mut parsed = ParsedArguments::new();
        let mut env_args = some_args.iter().map(T::to_string).peekable();
        fn is_short(word: &str) -> bool {
            word.starts_with('-') && !word.starts_with("--")
        }
        fn is_long(word: &str) -> bool {
            word.starts_with("--")
        }
        fn break_apart(word: &'_ str) -> impl Iterator<Item = String> + '_ {
            word.split("")
                .filter(|f| !f.is_empty() && !f.starts_with('-'))
                .map(|f| format!("-{}", f))
        }
        while env_args.peek().is_some() {
            let mut word = env_args.next().unwrap();
            let short = is_short(&word);
            let long = is_long(&word);
            let length = word.len();
            if long || (short && length < 3) {
                let negation = match long {
                    true => word.starts_with("--no-"),
                    false => word.starts_with("-no-"),
                };
                if negation {
                    if long {
                        word = str::replace(&word, "--no-", "--");
                    } else {
                        word = str::replace(&word, "-no-", "-");
                    };
                };
                let flag = self.is_flag(&word);
                if !flag && negation {
                    panic!("negation is only supported for flag type options: {}", word);
                };
                let arg = match self.get_arg(&word) {
                    Some(arg) => arg,
                    None => panic!("unrecognized argument: {} found", &word),
                };
                let name = arg.name.clone();
                if self.is_flag(&name) {
                    let val = ArgValue::Flag(!negation);
                    parsed.args.insert(name, val);
                    continue;
                };
                let mut value = Vec::<String>::new();
                while env_args.peek().is_some() {
                    let next = env_args.peek().unwrap();
                    if next.starts_with('-') {
                        break;
                    } else {
                        let next = env_args.next().unwrap();
                        value.push(next);
                    }
                }
                if self.is_text(&name) {
                    assert_eq!(
                        value.len(),
                        1usize,
                        "expected 1 value for {}, found: {}",
                        &word,
                        value.len()
                    );
                    parsed
                        .args
                        .insert(name, ArgValue::Text(value.pop().unwrap()));
                } else {
                    parsed.args.insert(name, ArgValue::Vector(value));
                };
            } else if short && length > 2 {
                let negation = word.starts_with("-no-");
                if negation {
                    word = str::replace(&word, "-no-", "-");
                    let flag = self.is_flag(&word);
                    if !flag {
                        panic!(
                            "negation is only supported for flag type arguments: {}",
                            &word
                        );
                    };
                    let arg = self.get_arg(&word).unwrap();
                    let name = arg.name.clone();
                    parsed.args.insert(name, ArgValue::Flag(false));
                } else {
                    let opts = break_apart(&word);
                    for item in opts {
                        assert!(self.contains_arg(&item));
                        let arg = self.get_arg(&item).unwrap();
                        let name = arg.name.clone();
                        let val = ArgValue::Flag(true);
                        parsed.args.insert(name, val);
                    }
                };
            }
        }
        assert!(self.parser.keys().all(|x| {
            if self.parser[x].required {
                if parsed.args.contains_key(x) {
                    true
                } else {
                    let short = &self.parser[x].short;
                    let long = &self.parser[x].long;
                    let mut iden = String::new();
                    if short.is_some() && long.is_some() {
                        iden = format!("{}/{}", short.as_ref().unwrap(), long.as_ref().unwrap());
                    } else if short.is_some() {
                        iden = short.as_ref().unwrap().to_string();
                    } else if long.is_some() {
                        iden = long.as_ref().unwrap().to_string();
                    };
                    panic!("argument: {} is required", iden)
                }
            } else {
                true
            }
        }));
        parsed
    }
}

mod argparser_tests {
    #[test]
    fn creates_argument() {
        let arg = super::Argument::with_type("vector")
            .name("beans")
            .long("--beans")
            .short("-b")
            .required(true);
        let mut parser = super::ArgParser::new();
        parser.add_argument(arg);
        assert!(parser.contains_arg("beans"));
        assert!(parser.contains_arg("-b"));
        assert!(parser.contains_arg("--beans"));
    }
    #[test]
    #[should_panic]
    fn no_options_panic() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .required(true),
        );
    }
    #[test]
    #[should_panic]
    fn no_name_panic() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .long("--beans")
                .short("-b")
                .required(true),
        );
    }
    #[test]
    #[should_panic]
    fn duplicate_name_panic() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .long("--bananas")
                .short("-x")
                .required(true),
        );
    }
    #[test]
    #[should_panic]
    fn duplicate_short_panic() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("bananas")
                .long("--bananas")
                .short("-b")
                .required(true),
        );
    }
    #[test]
    #[should_panic]
    fn duplicate_long_panic() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("bananas")
                .long("--beans")
                .short("-x")
                .required(true),
        );
    }
    #[test]
    fn get_args_works() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(false),
        );
        let long = parser.get_arg("--beans");
        let short = parser.get_arg("-b");
        let name = parser.get_arg("beans");
        assert!(name.is_some() && short.is_some() && long.is_some());
    }
    #[test]
    fn parses_correctly() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("text")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("vector")
                .name("bananas")
                .long("--bananas")
                .short("-s")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("carbon")
                .long("--carbon")
                .short("-c")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("hydrogen")
                .short("-H")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("kryptonite")
                .short("-p")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("iron")
                .short("-i")
                .required(false),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("corn")
                .long("--corn")
                .required(false),
        );
        let args = [
            "--bananas",
            "contains potassium",
            "contains calcium",
            "-b",
            "can cause gas",
            "-Hcp",
            // negation
            "-no-i",
            "--no-corn",
        ];
        let parsed = parser.parse_args(&args);
        assert!(parsed.args.get("bananas").is_some());
        assert!(parsed.args.get("beans").is_some());
        assert!(parsed.args.get("carbon").is_some());
        assert!(parsed.args.get("hydrogen").is_some());
        assert!(parsed.args.get("iron").is_some());
        assert!(parsed.args.get("corn").is_some());
        assert!(parsed.args.get("hydrocarbon").is_none());

        // assert negation
        let iron = parsed.args.get("iron").unwrap();
        let value = match iron {
            super::ArgValue::Flag(val) => val,
            _ => &true,
        };
        assert!(!value);
        let corn = parsed.args.get("corn").unwrap();
        let value = match corn {
            super::ArgValue::Flag(val) => val,
            _ => &true,
        };
        assert!(!value);
    }
    #[test]
    #[should_panic]
    fn asserts_required_args() {
        let mut parser = super::ArgParser::new();
        parser.add_argument(
            super::Argument::with_type("text")
                .name("beans")
                .long("--beans")
                .short("-b")
                .required(true),
        );
        parser.add_argument(
            super::Argument::with_type("flag")
                .name("bananas")
                .long("--bananas")
                .required(true),
        );
        let args = ["--beans", "hello"];
        let parsed = parser.parse_args(&args);
        dbg!(parsed);
    }
}
