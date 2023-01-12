use super::parser::slice::Slice;

#[derive(Debug)]
pub enum VariableType<S> {
    String(S),
    Number(f64),
    MessageReference(S),
    Markup { name: S },
    MarkupEnd { name: S },
    List(Vec<VariableType<S>>),
    // Custom, // Passing a date
}

impl<'a: 's, 's, S> VariableType<S>
where
    S: Slice<'s>,
{
    pub fn as_ref(&'a self) -> VariableType<&'s str> {
        match self {
            VariableType::String(s) => VariableType::String(s.as_str()),
            VariableType::Number(n) => VariableType::Number(*n),
            VariableType::MessageReference(s) => VariableType::MessageReference(s.as_str()),
            VariableType::Markup { name } => VariableType::Markup {
                name: name.as_str(),
            },
            VariableType::MarkupEnd { name } => VariableType::MarkupEnd {
                name: name.as_str(),
            },
            VariableType::List(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessagePart<S> {
    Literal(S),
    Markup { name: S },
    MarkupEnd { name: S },
    // Custom
}
