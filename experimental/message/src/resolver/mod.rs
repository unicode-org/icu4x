use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use std::borrow::Cow;
use std::collections::HashMap;

// 'v - lifetime of variables
// 'msgs - lifetime of messages
// 'msgsv - lifetime of messages value
pub struct Scope<'v, 'msgs, 'msgsv, S2, S4> {
    variables: Option<&'v HashMap<String, VariableType<S4>>>,
    messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<S2>>>,
}
// vars: Option<&'v HashMap<String, VariableType<S4>>>,
// msgs: Option<&'msgs HashMap<String, Vec<ast::PatternElement<S2>>>>,

impl<'v, 'msgs, 'msgsv, S, S4> Scope<'v, 'msgs, 'msgsv, S, S4> {
    pub fn new(
        variables: Option<&'v HashMap<String, VariableType<S4>>>,
        messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<S>>>,
    ) -> Self {
        Self {
            variables,
            messages,
        }
    }
}

trait MessagePartCollector<S> {
    fn push_part(&mut self, part: MessagePart<S>);
}

struct MessagePartsList<S>(Vec<MessagePart<S>>);
struct MessageString<'s>(Cow<'s, str>);
struct MessageSink<W>(W);

impl<S> MessagePartsList<S> {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl<'s> MessageString<'s> {
    pub fn new() -> Self {
        Self("".into())
    }
}

impl<W> MessageSink<W> {
    pub fn new(sink: W) -> Self {
        Self(sink)
    }
}

impl<S> MessagePartCollector<S> for MessagePartsList<S> {
    fn push_part(&mut self, part: MessagePart<S>) {
        self.0.push(part);
    }
}

impl<'s, S: 's + Slice<'s>> MessagePartCollector<S> for MessageString<'s> {
    fn push_part(&mut self, part: MessagePart<S>) {
        if self.0.is_empty() {
            match part {
                MessagePart::Literal(l) => {
                    self.0 = l.into_cow();
                }
            }
        } else {
            let new_part = match part {
                MessagePart::Literal(l) => l.into_cow(),
            };
            if !new_part.is_empty() {
                self.0.to_mut().push_str(&new_part);
            }
        }
    }
}

impl<'s, S: 's + Slice<'s>, W: std::fmt::Write> MessagePartCollector<S> for MessageSink<W> {
    fn push_part(&mut self, part: MessagePart<S>) {
        match part {
            MessagePart::Literal(l) => self.0.write_str(l.as_str()).unwrap(),
        }
    }
}

pub struct Resolver<S, S2, S3, S4> {
    p1: std::marker::PhantomData<S>,
    p2: std::marker::PhantomData<S2>,
    p3: std::marker::PhantomData<S3>,
    p4: std::marker::PhantomData<S4>,
}

impl<'m, 'mv, 'v, 'vv, 'msgs, 'msgsv, 'mv2, 'p, 's, S, S2, S3, S4> Resolver<S, S2, S3, S4>
where
    S: Slice<'mv>,
    S2: Slice<'mv2>,
    S3: 'v + Slice<'p>,
    S4: Slice<'vv>,
    'mv: 'p,
    'mv2: 'p,
    'v: 'p,
    'm: 'p,
    'msgs: 'p,
    'vv: 'p,
{
    pub fn resolve_to_parts(
        msg: &'m ast::Message<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
    ) -> Vec<MessagePart<S3>> {
        let mut collector = MessagePartsList::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_string(
        msg: &'m ast::Message<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
    ) -> Cow<'p, str> {
        let mut collector = MessageString::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_sink<W: std::fmt::Write>(
        msg: &'m ast::Message<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        sink: W,
    ) {
        let mut collector = MessageSink::new(sink);
        Self::resolve_message_to_collector(msg, scope, &mut collector);
    }

    fn resolve_message_to_collector<C>(
        msg: &'m ast::Message<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<S3>,
    {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        for pe in &pattern.body {
            Self::resolve_pattern_element(pe, scope, collector);
        }
    }

    fn resolve_pattern_element<C>(
        pe: &'m ast::PatternElement<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<S3>,
    {
        match pe {
            ast::PatternElement::Text(s) => {
                collector.push_part(MessagePart::Literal(S3::from_str(s.as_str())))
            }
            ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, scope, collector),
        }
    }

    fn resolve_placeholder<C>(
        placeholder: &'m ast::Placeholder<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<S3>,
    {
        match placeholder {
            ast::Placeholder::Markup { name, options } => todo!(),
            ast::Placeholder::MarkupEnd { name } => todo!(),
            ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope, collector),
        }
    }

    fn resolve_expression<C>(
        exp: &'m ast::Expression<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<S3>,
    {
        match exp {
            ast::Expression::Operand {
                operand,
                annotation,
            } => match operand {
                ast::Operand::Literal(l) => {
                    collector.push_part(MessagePart::Literal(S3::from_str(l.value.as_str())))
                }
                ast::Operand::Variable(v) => Self::resolve_variable(v, scope, collector),
            },
            ast::Expression::Annotation(_) => todo!(),
        }
    }

    fn resolve_variable<C>(
        variable: &'m S,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2, S4>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<S3>,
    {
        if let Some(variables) = scope.variables {
            if let Some(v) = variables.get(variable.as_str()) {
                match v {
                    VariableType::String(s) => {
                        collector.push_part(MessagePart::Literal(S3::from_str(s.as_str())))
                    }
                    VariableType::MessageReference(id) => {
                        if let Some(messages) = scope.messages {
                            if let Some(msg) = messages.get(id.as_str()) {
                                Resolver::resolve_message_to_collector(*msg, scope, collector);
                            } else {
                                todo!()
                            }
                        } else {
                            todo!()
                        }
                    }
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::parser::Parser;
    use super::super::types::{MessagePart, VariableType};
    use super::ast;
    use super::{Resolver, Scope};
    use smallvec::SmallVec;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));
        let scope = Scope::new(Some(&variables), None);
        let string = Resolver::<_, &str, &str, _>::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Hello World");
    }

    #[test]
    fn stay_borrowed_check() {
        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![ast::PatternElement::Text("Hello World")]),
            }),
        };

        let scope = Scope::<&str, &str>::new(None, None);
        let string = Resolver::<_, _, &str, _>::resolve_to_string(&msg, &scope);

        assert_eq!(string, Cow::Borrowed("Hello World"));

        let scope = Scope::<&str, &str>::new(None, None);
        let parts = Resolver::<_, _, &str, _>::resolve_to_parts(&msg, &scope);

        assert_eq!(parts, vec![MessagePart::Literal("Hello World"),]);

        let mut sink = String::new();
        let scope = Scope::<&str, &str>::new(None, None);
        Resolver::<_, _, &str, _>::resolve_to_sink(&msg, &scope, &mut sink);

        assert_eq!(sink, "Hello World");
    }

    #[test]
    fn lifetimes_check() {
        let parser = Parser::new("{Hello World{$name}{$creature}}");
        let msg = parser.parse().unwrap();
        let parser = Parser::new("{Dragon}".to_string());
        let creature_msg = parser.parse().unwrap();
        let mut msgs = HashMap::new();
        msgs.insert("dragon".to_string(), &creature_msg);

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));
        variables.insert("creature".into(), VariableType::MessageReference("dragon"));
        let scope = Scope::new(Some(&variables), Some(&msgs));
        let parts = Resolver::resolve_to_parts(&msg, &scope);

        assert_eq!(
            parts,
            vec![
                MessagePart::Literal("Hello World"),
                MessagePart::Literal("John"),
                MessagePart::Literal("Dragon"),
            ]
        );
    }

    #[test]
    fn allocate_check() {
        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![
                    ast::PatternElement::Text("Hello "),
                    ast::PatternElement::Text("World"),
                ]),
            }),
        };

        let scope = Scope::<&str, &str>::new(None, None);
        let string = Resolver::<_, _, &str, _>::resolve_to_string(&msg, &scope);

        assert_eq!(string, Cow::<str>::Owned(String::from("Hello World")));

        let scope = Scope::<&str, &str>::new(None, None);
        let parts = Resolver::resolve_to_parts(&msg, &scope);

        assert_eq!(
            parts,
            vec![
                MessagePart::Literal("Hello "),
                MessagePart::Literal("World"),
            ]
        );
    }

    #[test]
    fn variable_check() {
        let source = "{{$name}}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));
        let scope = Scope::<&str, _>::new(Some(&variables), None);
        let string = Resolver::<_, _, &str, _>::resolve_to_string(&msg, &scope);

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
            VariableType::MessageReference("creature-dragon"),
        );

        let scope = Scope::new(Some(&variables), Some(&msgs));
        let string = Resolver::<_, _, &str, _>::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Dragon killed you.");
    }
}
