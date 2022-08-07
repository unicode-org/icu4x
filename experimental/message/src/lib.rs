pub mod ast;
pub mod parser;
pub mod resolver;
pub mod types;

pub struct MessageFormat {}

impl MessageFormat {
    pub fn new() -> Self {
        Self {}
    }

    pub fn format<'s>(&self, msg: &ast::Message<&'s str>) -> String {
        panic!();
    }
}

#[cfg(test)]
mod test {
    use super::parser::Parser;
    use super::resolver::{Resolver, Scope};
    use super::types::VariableType;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John".into()));
        let scope = Scope::new(variables);
        let string = Resolver::resolve_to_string(&msg, scope);

        assert_eq!(string, "Hello World");
    }

    #[test]
    fn variable_check() {
        let source = "{{$name}}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John".into()));
        let scope = Scope::new(variables);
        let string = Resolver::resolve_to_string(&msg, scope);

        assert_eq!(string, "John");
    }
}
