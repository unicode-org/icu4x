mod collector;
mod scope;

use collector::*;
pub use scope::Scope;

use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use std::borrow::Cow;

// MV - message value type
// VARSV - variables value type
// MSGSV - messages value type
// MPV - message parts value type
pub struct Resolver<MV, VARSV, MSGSV, MPV> {
    p1: std::marker::PhantomData<MV>,
    p2: std::marker::PhantomData<VARSV>,
    p3: std::marker::PhantomData<MSGSV>,
    p4: std::marker::PhantomData<MPV>,
}

// 'm - message lifetime
// 'mv - message value lifetime
// 'varsm - variables map lifetime
// 'varsv - variables values lifetime
// 'msgsm - messages map lifetime
// 'msgsmv - messages map value lifetime
// 'msgsv - messages value lifetime
// 'scope - scope lifetime
// 'mpv - message parts value lifetime
impl<'m, 'mv, 'varsm, 'varsv, 'msgsm, 'msgsmv, 'msgsv, 'scope, 'mpv, MV, VARSV, MSGSV, MPV>
    Resolver<MV, VARSV, MSGSV, MPV>
where
    MV: Slice<'mv>,
    VARSV: Slice<'varsv>,
    MSGSV: Slice<'msgsv>,
    MPV: 'mpv + Slice<'mpv>,
    // 'm: 'mv,
    'mv: 'mpv,
    'msgsv: 'mpv,
    'varsv: 'mpv,
    // 'm: 'mpv,
    // 'varsm: 'mpv,
    // 'msgsm: 'mpv,
{
    pub fn resolve_to_parts(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
    ) -> Vec<MessagePart<MPV>> {
        let mut collector = MessagePartsList::new();
        // let s: &'mv str = match &msg.value {
        //     ast::MessageValue::Pattern(p) => {
        //         match p.body.first().unwrap() {
        //             ast::PatternElement::Text(s) => s.as_str(),
        //             ast::PatternElement::Placeholder(_) => todo!(),
        //         }
        //     },
        //     ast::MessageValue::Select(_) => todo!(),
        // };
        // collector.push_part(MessagePart::Literal(MPV::from_str(s)));
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_string(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
    ) -> Cow<'mpv, str> {
        let mut collector = MessageString::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_sink<W: std::fmt::Write>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        sink: W,
    ) {
        let mut collector = MessageSink::new(sink);
        Self::resolve_message_to_collector(msg, scope, &mut collector);
    }

    fn resolve_message_to_collector<C>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
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
        pe: &'m ast::PatternElement<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
    {
        match pe {
            ast::PatternElement::Text(s) => {
                collector.push_part(MessagePart::Literal(MPV::from_slice(s)))
            }
            ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, scope, collector),
        }
    }

    fn resolve_placeholder<C>(
        placeholder: &'m ast::Placeholder<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
    {
        match placeholder {
            ast::Placeholder::Markup { name, options } => todo!(),
            ast::Placeholder::MarkupEnd { name } => todo!(),
            ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope, collector),
        }
    }

    fn resolve_expression<C>(
        exp: &'m ast::Expression<MV>,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
    {
        match exp {
            ast::Expression::Operand {
                operand,
                annotation,
            } => match operand {
                ast::Operand::Literal(l) => {
                    collector.push_part(MessagePart::Literal(MPV::from_slice(&l.value)))
                }
                ast::Operand::Variable(v) => Self::resolve_variable(v, scope, collector),
            },
            ast::Expression::Annotation(_) => todo!(),
        }
    }

    fn resolve_variable<C>(
        variable: &'m MV,
        scope: &'scope Scope<'varsm, 'msgsm, 'msgsmv, VARSV, MSGSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
    {
        if let Some(variables) = scope.variables {
            if let Some(v) = variables.get(variable.as_str()) {
                match v {
                    VariableType::String(s) => {
                        collector.push_part(MessagePart::Literal(MPV::from_slice(s)))
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
        let string = Resolver::<_, _, &str, &str>::resolve_to_string(&msg, &scope);

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

        let scope = Scope::new(None, None);
        let string = Resolver::<_, &str, &str, &str>::resolve_to_string(&msg, &scope);

        assert!(matches!(string, Cow::Borrowed("Hello World")));

        let scope = Scope::<&str, &str>::new(None, None);
        let parts = Resolver::<_, _, &str, _>::resolve_to_parts(&msg, &scope);

        assert_eq!(parts, vec![MessagePart::Literal("Hello World"),]);

        let mut sink = String::new();
        let scope = Scope::new(None, None);
        Resolver::<_, &str, &str, &str>::resolve_to_sink(&msg, &scope, &mut sink);

        assert_eq!(sink, "Hello World");
    }

    #[test]
    fn lifetimes_check() {
        let parser = Parser::new("{Hello World{$name}{$creature}}");
        let msg = parser.parse().unwrap();
        let parser = Parser::new("{Dragon}");
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

        let parser = Parser::new("{{$name}}");
        let msg = parser.parse().unwrap();
        let string = Resolver::<_, _, _, &str>::resolve_to_string(&msg, &scope);
        assert!(matches!(string, Cow::Borrowed("John")));

        let parser = Parser::new("{{$creature}}");
        let msg = parser.parse().unwrap();
        let string = Resolver::<_, _, _, &str>::resolve_to_string(&msg, &scope);
        assert!(matches!(string, Cow::Borrowed("Dragon")));
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
        let string = Resolver::<_, _, _, &str>::resolve_to_string(&msg, &scope);

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
        let scope = Scope::new(Some(&variables), None);
        let string = Resolver::<_, _, &str, &str>::resolve_to_string(&msg, &scope);

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
        let string = Resolver::<_, _, _, &str>::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Dragon killed you.");
    }
}
