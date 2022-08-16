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

#[derive(Debug, PartialEq)]
pub enum MessagePart<S> {
    Literal(S),
    // Markup { name: S },
    // MarkupEnd { name: S },
}
