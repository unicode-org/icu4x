mod collector;
mod scope;

use collector::*;
pub use scope::Scope;

use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use crate::MF2Function;
use std::borrow::Cow;

// MV - message value type
// VARSV - variables value type
// MSGSV - messages value type
// MPV - message parts value type
pub struct Resolver<MV, VARSV, MPV> {
    p1: std::marker::PhantomData<MV>,
    p2: std::marker::PhantomData<VARSV>,
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
impl<'b, 'm, 'mv, 'varsm, 'varsv, 'mf, 'scope, 'mpv, MV, VARSV, MPV> Resolver<MV, VARSV, MPV>
where
    MV: Slice<'mv>,
    VARSV: Slice<'varsv>,
    MPV: 'mpv + Slice<'mpv>,
    'mv: 'mpv,
    'varsv: 'mpv,
    'varsm: 'varsv,
{
    pub fn resolve_to_parts(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
    ) -> Vec<MessagePart<MPV>> {
        let mut collector = MessagePartsList::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_string(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
    ) -> Cow<'mpv, str> {
        let mut collector = MessageString::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_sink<W: std::fmt::Write>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
        sink: W,
    ) {
        let mut collector = MessageSink::new(sink);
        Self::resolve_message_to_collector(msg, scope, &mut collector);
    }

    fn resolve_message_to_collector<C>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
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
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
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
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
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
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
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
                ast::Operand::Variable(v) => {
                    let var = Self::get_variable(v, scope).unwrap();
                    if let Some(annotation) = annotation {
                        let func = Self::get_function(&annotation.function, scope).unwrap();
                        let v: VariableType<&'varsv str> = var.as_ref();
                        let result = func(&v, scope.mf);
                        for item in result {
                            match item {
                                MessagePart::Literal(s) => {
                                    let s: Cow<str> = Cow::Owned(s);
                                    collector.push_part(MessagePart::Literal(MPV::from_cow(s)))
                                }
                                MessagePart::Markup { name } => {
                                    collector.push_part(MessagePart::Markup {
                                        name: MPV::from_slice(&name.to_owned()),
                                    });
                                }
                                MessagePart::MarkupEnd { name } => {
                                    collector.push_part(MessagePart::MarkupEnd {
                                        name: MPV::from_slice(&name.to_owned()),
                                    });
                                }
                            }
                        }
                    } else {
                        Self::resolve_variable(var, scope, collector);
                    }
                }
            },
            ast::Expression::Annotation(_) => todo!(),
        }
    }

    fn get_variable(
        variable: &'m MV,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
    ) -> Option<&'varsm VariableType<VARSV>> {
        scope.variables.and_then(|vars| vars.get(variable.as_str()))
    }

    fn get_function(
        function: &'m MV,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
    ) -> Option<&'mf MF2Function<'b>> {
        scope.mf.functions.get(function.as_str())
    }

    fn resolve_variable<C, V>(
        var: &VariableType<V>,
        scope: &'scope Scope<'b, 'mf, 'varsm, VARSV>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<MPV>,
        V: Slice<'varsv>,
    {
        match var {
            VariableType::String(s) => {
                collector.push_part(MessagePart::Literal(MPV::from_slice(s.to_owned())))
            }
            VariableType::Number(n) => {
                let result = format!("{n}");
                collector.push_part(MessagePart::Literal(MPV::from_slice(&result)))
            }
            VariableType::MessageReference(id) => {
                // if let Some(messages) = scope.messages {
                //     if let Some(msg) = messages.get(id.as_str()) {
                //         Resolver::resolve_message_to_collector(*msg, scope, collector);
                //     } else {
                //         todo!()
                //     }
                // } else {
                //     todo!()
                // }
            }
            VariableType::List(v) => {
                for item in v {
                    Self::resolve_variable(item, scope, collector);
                }
            }
            VariableType::Markup { name } => {
                collector.push_part(MessagePart::Markup {
                    name: MPV::from_slice(name.to_owned()),
                });
            }
            VariableType::MarkupEnd { name } => {
                collector.push_part(MessagePart::MarkupEnd {
                    name: MPV::from_slice(name.to_owned()),
                });
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::parser::Parser;
    use super::super::types::{MessagePart, VariableType};
    use super::ast;
    use super::{Resolver, Scope};
    use crate::MessageFormat;
    use icu_locid::locale;
    use smallvec::SmallVec;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let mf = MessageFormat::new(locale!("en-US"));
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));
        let scope = Scope::new(&mf, Some(&variables));
        let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Hello World");
    }

    #[test]
    fn stay_borrowed_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![ast::PatternElement::Text("Hello World")]),
            }),
        };

        let scope = Scope::new(&mf, None);
        let string = Resolver::<_, &str, &str>::resolve_to_string(&msg, &scope);

        assert!(matches!(string, Cow::Borrowed("Hello World")));

        let scope = Scope::<&str>::new(&mf, None);
        let parts = Resolver::<_, _, &str>::resolve_to_parts(&msg, &scope);

        assert_eq!(parts, vec![MessagePart::Literal("Hello World"),]);

        let mut sink = String::new();
        let scope = Scope::<&str>::new(&mf, None);
        Resolver::<_, _, &str>::resolve_to_sink(&msg, &scope, &mut sink);

        assert_eq!(sink, "Hello World");
    }

    // #[test]
    // fn lifetimes_check() {
    //     let mf = MessageFormat::new();
    //
    //     let parser = Parser::new("{Hello World{$name}{$creature}}");
    //     let msg = parser.parse().unwrap();
    //     // let parser = Parser::new("{Dragon}");
    //     // let creature_msg = parser.parse().unwrap();
    //     // let mut msgs = HashMap::new();
    //     // msgs.insert("dragon".to_string(), &creature_msg);
    //
    //     let mut variables = HashMap::new();
    //     variables.insert("name".into(), VariableType::String("John"));
    //     variables.insert("creature".into(), VariableType::MessageReference("dragon"));
    //     let scope = Scope::new(&mf, Some(&variables));
    //     let parts = Resolver::resolve_to_parts(&msg, &scope);
    //
    //     assert_eq!(
    //         parts,
    //         vec![
    //             MessagePart::Literal("Hello World"),
    //             MessagePart::Literal("John"),
    //             MessagePart::Literal("Dragon"),
    //         ]
    //     );
    //
    //     let parser = Parser::new("{{$name}}");
    //     let msg = parser.parse().unwrap();
    //     let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);
    //     assert!(matches!(string, Cow::Borrowed("John")));
    //
    //     let parser = Parser::new("{{$creature}}");
    //     let msg = parser.parse().unwrap();
    //     let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);
    //     assert!(matches!(string, Cow::Borrowed("Dragon")));
    // }

    #[test]
    fn allocate_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![
                    ast::PatternElement::Text("Hello "),
                    ast::PatternElement::Text("World"),
                ]),
            }),
        };

        let scope = Scope::<&str>::new(&mf, None);
        let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);

        assert_eq!(string, Cow::<str>::Owned(String::from("Hello World")));

        let scope = Scope::<&str>::new(&mf, None);
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
        let mf = MessageFormat::new(locale!("en-US"));

        let source = "{{$name}}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));
        let scope = Scope::new(&mf, Some(&variables));
        let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);

        assert_eq!(string, "John");
    }
}
