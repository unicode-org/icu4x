use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use std::collections::HashMap;

pub struct Scope<'h, 'm, S> {
    variables: HashMap<String, VariableType>,
    messages: Option<&'h HashMap<String, &'m ast::Message<S>>>,
}

impl<'h, 'm, S> Scope<'h, 'm, S> {
    pub fn new(
        variables: Option<HashMap<String, VariableType>>,
        messages: Option<&'h HashMap<String, &'m ast::Message<S>>>,
    ) -> Self {
        Self {
            variables: variables.unwrap_or_default(),
            messages,
        }
    }
}

pub struct Resolver {}

impl<'h, 'm> Resolver {
    pub fn resolve_to_parts<S: Slice>(
        msg: &ast::Message<S>,
        scope: &Scope<'h, 'm, S>,
    ) -> Vec<MessagePart<S>> {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        if pattern.body.len() == 1 {
            todo!()
        } else {
            todo!()
        }
    }

    pub fn resolve_to_string<S: Slice>(
        msg: &ast::Message<S>,
        scope: &Scope<'h, 'm, S>,
    ) -> S::Output {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        if pattern.body.len() == 1 {
            let pe = pattern.body.first().unwrap();
            Self::resolve_pattern_element(pe, scope)
        } else {
            let mut result = String::new();
            for pe in &pattern.body {
                let part = Self::resolve_pattern_element(pe, scope);
                result.push_str(part.as_ref());
            }
            S::from_string(result)
        }
    }

    fn resolve_pattern_element<S: Slice>(
        pe: &ast::PatternElement<S>,
        scope: &Scope<'h, 'm, S>,
    ) -> S::Output {
        match pe {
            ast::PatternElement::Text(s) => s.as_output(),
            ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, scope),
        }
    }

    fn resolve_placeholder<S: Slice>(
        placeholder: &ast::Placeholder<S>,
        scope: &Scope<'h, 'm, S>,
    ) -> S::Output {
        match placeholder {
            ast::Placeholder::Markup { name, options } => todo!(),
            ast::Placeholder::MarkupEnd { name } => todo!(),
            ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope),
        }
    }

    fn resolve_expression<S: Slice>(
        exp: &ast::Expression<S>,
        scope: &Scope<'h, 'm, S>,
    ) -> S::Output {
        match exp {
            ast::Expression::Operand {
                operand,
                annotation,
            } => match operand {
                ast::Operand::Literal(l) => l.value.as_output(),
                ast::Operand::Variable(v) => {
                    if let Some(v) = scope.variables.get(v.as_str()) {
                        match v {
                            VariableType::String(s) => S::output_from_str(s),
                            VariableType::Number(_) => todo!(),
                            VariableType::MessageReference(id) => {
                                if let Some(messages) = scope.messages {
                                    if let Some(msg) = messages.get(id.as_str()) {
                                        Self::resolve_to_string(msg, scope)
                                    } else {
                                        todo!()
                                    }
                                } else {
                                    todo!()
                                }
                            }
                            VariableType::Markup { name } => todo!(),
                            VariableType::MarkupEnd { name } => todo!(),
                            VariableType::List(_) => todo!(),
                        }
                    } else {
                        todo!()
                    }
                }
            },
            ast::Expression::Annotation(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::parser::Parser;
    use super::super::types::VariableType;
    use super::{Resolver, Scope};
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John".into()));
        let scope = Scope::new(Some(variables), None);
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Hello World");
    }

    #[test]
    fn variable_check() {
        let source = "{{$name}}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John".into()));
        let scope = Scope::new(Some(variables), None);
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, "John");
    }

    #[test]
    fn ref_msg_check() {
        let parser = Parser::new("{Dragon}");
        let dragon_msg = parser.parse().unwrap();

        let parser = Parser::new("{Golem}");
        let golem_msg = parser.parse().unwrap();

        let source = "{{$monster} killed you.}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut msgs = HashMap::new();
        msgs.insert("creature-dragon".to_string(), &dragon_msg);
        msgs.insert("creature-golem".to_string(), &golem_msg);

        let mut variables = HashMap::new();
        variables.insert(
            "monster".into(),
            VariableType::MessageReference("creature-dragon".into()),
        );

        let scope = Scope::new(Some(variables), Some(&msgs));
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Dragon killed you.");
    }
}
