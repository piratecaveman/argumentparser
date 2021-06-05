use crate::defarg::atypes::AType;

#[derive(Debug, Clone)]
pub struct Argument {
    pub atype: AType,
    pub name: String,
    pub short: Option<String>,
    pub long: Option<String>,
    pub required: bool,
}

impl Default for Argument {
    fn default() -> Self {
        Argument {
            atype: AType::default(),
            name: String::new(),
            short: None,
            long: None,
            required: false,
        }
    }
}

impl Argument {
    pub fn new<T: ToString>(atype: T) -> Argument {
        Argument::with_type(atype)
    }
    pub fn with_type<T: ToString>(atype: T) -> Argument {
        Argument {
            atype: AType::from(atype.to_string()),
            ..Argument::default()
        }
    }
    pub fn name<T: ToString>(mut self, name: T) -> Argument {
        self.name = name.to_string();
        self
    }
    pub fn short<T: ToString>(mut self, short: T) -> Argument {
        let short = short.to_string();
        assert!(short.starts_with('-'), r#"argument should start with "-""#);
        assert!(
            !short.starts_with("--"),
            r#"short argument should not start with "--""#
        );
        assert!(
            short.len().eq(&2usize),
            "short argument is too long or null"
        );
        self.short = Some(short);
        self
    }
    pub fn long<T: ToString>(mut self, long: T) -> Argument {
        let long = long.to_string();
        assert!(
            long.starts_with("--"),
            r#"long argument should start with "--""#
        );
        assert!(
            long.len().gt(&2usize),
            "long argument is too short consider switching it to short"
        );
        self.long = Some(long);
        self
    }
    pub fn required(mut self, required: bool) -> Argument {
        self.required = required;
        self
    }
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && (self.short.is_some() || self.long.is_some())
    }
}

mod arg_tests {
    #[test]
    fn works_as_intended() {
        let our_arg = super::Argument::with_type("flag")
            .name("radio")
            .long("--radio")
            .short("-r")
            .required(false);
        assert!(our_arg.is_valid());
    }
    #[test]
    #[should_panic]
    fn bad_arg_panic() {
        let our_arg = super::Argument::with_type("vector").required(false);
        assert!(our_arg.is_valid());
    }
    #[test]
    #[should_panic]
    fn bad_short_option_panic() {
        let our_arg = super::Argument::with_type("vector")
            .required(false)
            .short("--krypton");
        dbg!(our_arg);
    }
    #[test]
    #[should_panic]
    fn bad_long_option_panic() {
        let our_arg = super::Argument::with_type("vector")
            .required(false)
            .long("-krypton");
        dbg!(our_arg);
    }
}
