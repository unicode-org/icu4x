use smallvec::SmallVec;

#[derive(Debug, PartialEq)]
pub struct Message<S> {
    pub declarations: SmallVec<[Declaration<S>; 1]>,
    pub value: MessageValue<S>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration<S> {
    pub variable: S,
    pub expression: Expression<S>,
}

#[derive(Debug, PartialEq)]
pub enum MessageValue<S> {
    Pattern(Pattern<S>),
    Select(Box<Select<S>>),
}

#[derive(Debug, PartialEq)]
pub struct Select<S> {
    pub selector: SmallVec<[Expression<S>; 1]>,
    pub variants: SmallVec<[Variant<S>; 3]>,
}

#[derive(Debug, PartialEq)]
pub struct Variant<S> {
    pub key: SmallVec<[VariantKey<S>; 1]>,
    pub pattern: Pattern<S>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern<S> {
    pub body: SmallVec<[PatternElement<S>; 3]>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement<S> {
    Text(S),
    Placeholder(Placeholder<S>),
}

#[derive(Debug, PartialEq)]
pub enum Placeholder<S> {
    Markup {
        name: S,
        options: SmallVec<[Option<S>; 1]>,
    },
    MarkupEnd {
        name: S,
    },
    Expression(Expression<S>),
}

#[derive(Debug, PartialEq)]
pub enum Expression<S> {
    Operand {
        operand: Operand<S>,
        annotation: std::option::Option<Annotation<S>>,
    },
    Annotation(Annotation<S>),
}

#[derive(Debug, PartialEq)]
pub enum Operand<S> {
    Literal(Literal<S>),
    Variable(S),
}

#[derive(Debug, PartialEq)]
pub struct Annotation<S> {
    pub function: S,
    pub options: SmallVec<[Option<S>; 1]>,
}

#[derive(Debug, PartialEq)]
pub struct Literal<S> {
    pub value: S,
}

#[derive(Debug, PartialEq)]
pub enum VariantKey<S> {
    Literal(Literal<S>),
    Asterisk,
}

#[derive(Debug, PartialEq)]
pub struct Option<S> {
    name: S,
    value: OptionValue<S>,
}

#[derive(Debug, PartialEq)]
pub enum OptionValue<S> {
    Literal(Literal<S>),
    Variable(S),
}
