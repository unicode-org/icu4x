use super::parser::slice::Slice;

#[derive(Debug)]
pub enum VariableType {
    String(String),
    Number(f64),
    MessageReference(String),
    Markup { name: String },
    MarkupEnd { name: String },
    List(Vec<VariableType>),
}

#[derive(Debug, PartialEq)]
pub enum MessagePart<S>
where
    S: Slice,
{
    Literal(S::Output),
    Markup { name: S::Output },
    MarkupEnd { name: S::Output },
}
