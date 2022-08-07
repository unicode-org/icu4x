use super::ast;
use super::types::VariableType;
use std::borrow::Cow;
use std::collections::HashMap;

pub struct Scope {
    variables: HashMap<String, VariableType>,
}

impl Scope {
    pub fn new(variables: HashMap<String, VariableType>) -> Self {
        Self { variables }
    }
}

pub struct Resolver {}

impl Resolver {
    pub fn resolve_to_string<'s>(msg: &ast::Message<&'s str>, scope: Scope) -> Cow<'s, str> {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        if pattern.body.len() == 1 {
            let element = pattern.body.first().unwrap();
            match element {
                ast::PatternElement::Text(s) => (*s).into(),
                ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, &scope),
            }
        } else {
            todo!()
        }
    }

    fn resolve_placeholder<'s>(
        placeholder: &ast::Placeholder<&'s str>,
        scope: &Scope,
    ) -> Cow<'s, str> {
        match placeholder {
            ast::Placeholder::Markup { name, options } => todo!(),
            ast::Placeholder::MarkupEnd { name } => todo!(),
            ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope),
        }
    }

    fn resolve_expression<'s>(exp: &ast::Expression<&'s str>, scope: &Scope) -> Cow<'s, str> {
        match exp {
            ast::Expression::Operand {
                operand,
                annotation,
            } => match operand {
                ast::Operand::Literal(l) => l.value.into(),
                ast::Operand::Variable(v) => {
                    if let Some(v) = scope.variables.get(*v) {
                        match v {
                            VariableType::String(s) => s.clone().into(),
                            VariableType::Number(_) => todo!(),
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
