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
pub enum MessagePart<'s, S>
where
    S: Slice<'s>,
{
    Literal(S),
    Markup { name: S },
    MarkupEnd { name: S },
    PhantomData(std::marker::PhantomData<&'s str>),
}
