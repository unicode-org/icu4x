pub mod ast;
pub mod parser;
pub mod resolver;
pub mod types;
use parser::{slice::Slice, Parser};
use resolver::{Resolver, Scope};
use std::collections::HashMap;
use types::VariableType;

#[derive(Default)]
pub struct MessageFormat<'m, S> {
    pub messages: HashMap<String, &'m ast::Message<S>>,
}

impl<'m, S> MessageFormat<'m, S>
where
    S: Slice,
{
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
        }
    }

    pub fn format(
        &self,
        msg: &ast::Message<S>,
        variables: Option<HashMap<String, VariableType>>,
    ) -> S::Output {
        let scope = Scope::new(variables, Some(&self.messages));
        Resolver::resolve_to_string(msg, &scope)
    }

    pub fn format_from_source(
        &self,
        source: S,
        variables: Option<HashMap<String, VariableType>>,
    ) -> S::Output {
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();
        let scope = Scope::new(variables, Some(&self.messages));
        Resolver::resolve_to_string(&msg, &scope)
    }
}

#[cfg(test)]
mod test {
    use super::parser::Parser;
    use super::types::VariableType;
    use super::MessageFormat;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let mf = MessageFormat::new();

        let result = mf.format_from_source("{Hello World}", None);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn variable_check() {
        let mf = MessageFormat::new();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John".into()));

        let result = mf.format_from_source("{{$name}}", Some(variables));
        assert_eq!(result, "John");
    }

    #[test]
    fn ref_msg_check() {
        let mut mf = MessageFormat::new();

        let parser = Parser::new("{Dragon}");
        let dragon_msg = parser.parse().unwrap();
        let parser = Parser::new("{Golem}");
        let golem_msg = parser.parse().unwrap();

        mf.messages
            .insert("creature-dragon".to_string(), &dragon_msg);
        mf.messages.insert("creature-golem".to_string(), &golem_msg);

        let mut variables = HashMap::new();
        variables.insert(
            "monster".into(),
            VariableType::MessageReference("creature-dragon".into()),
        );

        let result = mf.format_from_source("{{$monster} killed you.}", Some(variables));
        assert_eq!(result, "Dragon killed you.");
    }
}
