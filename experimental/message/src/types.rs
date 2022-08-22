use super::parser::slice::Slice;

#[derive(Debug)]
pub enum VariableType<S> {
    String(S),
    // Number(f64),
    MessageReference(S),
    // Markup { name: String },
    // MarkupEnd { name: String },
    // List(Vec<VariableType>),
}

impl<'a: 's, 's, S> VariableType<S>
where
    S: Slice<'s>,
{
    pub fn as_ref(&'a self) -> VariableType<&'s str> {
        match self {
            VariableType::String(s) => VariableType::String(s.as_str()),
            VariableType::MessageReference(s) => VariableType::MessageReference(s.as_str()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessagePart<S> {
    Literal(S),
    // Markup { name: S },
    // MarkupEnd { name: S },
}
