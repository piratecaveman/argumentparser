use std::collections::HashSet;

use crate::argument::types::ArgumentType;

#[derive(Debug, Clone, Default)]
pub struct Argument {
    pub name: String,
    pub invoke_with: HashSet<String>,
    pub required: bool,
    argument_type: ArgumentType,
}

impl Argument {
    /// create a new blank argument with specified type  
    /// accepted types are "flag" "word" and  "vector"  
    pub fn with_type(argument_type: &str) -> Argument {
        Argument {
            argument_type: ArgumentType::from(argument_type),
            ..Argument::default()
        }
    }
    /// give a name to the argument  
    /// this name will be used to access the parsed argument
    /// name must be unique for each argument
    pub fn name(mut self, name: &str) -> Argument {
        if !self.name.is_empty() {
            panic!("name already specified")
        };
        self.name = name.to_string();
        self.invoke_with.insert(name.to_string());
        self
    }
    /// invoactor is the keyword that will be looked for when parsing the arguments  
    /// invocators mut be unique and different for each argument
    /// argument name is automatically used as an invocator  
    pub fn invoke_with(mut self, option: &str) -> Argument {
        self.invoke_with.insert(option.to_string());
        self
    }
    /// make the argument optional or required
    pub fn required(mut self, required: bool) -> Argument {
        self.required = required;
        self
    }
    /// compare the given string and returns true if it matches any of the argument's invocators  
    /// the string should be either the name or one of the invocators  
    pub fn same_as(&self, identity: &str) -> bool {
        self.invoke_with.contains(identity)
    }
    /// if an argument has a name and at least one invocator, it is considered a valid argument  
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
    pub fn get_type(&self) -> &str {
        self.argument_type.as_ref()
    }
}

mod argument_tests {
    #[test]
    fn same_as() {
        let oranges = super::Argument::with_type("flag")
            .name("oranges")
            .invoke_with("--oranges")
            .invoke_with("-o")
            .invoke_with("set-orange")
            .required(false);
        assert!(oranges.is_valid());
        assert!(oranges.same_as("oranges"));
        assert!(oranges.same_as("--oranges"));
        assert!(oranges.same_as("-o"));
        assert!(oranges.same_as("set-orange"));
    }
    #[test]
    #[should_panic]
    fn invalid_panic() {
        let apples = super::Argument::with_type("vector");
        assert!(apples.is_valid(), "no name/invocator provided");
    }
}
