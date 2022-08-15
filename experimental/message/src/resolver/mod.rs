use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use std::borrow::Cow;
use std::collections::HashMap;

// 'v - lifetime of variables
// 'msgs - lifetime of messages
// 'msgsv - lifetime of messages value
pub struct Scope<'v, 'msgs, 'msgsv, S2> {
    variables: Option<&'v HashMap<String, VariableType>>,
    messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<S2>>>,
}

impl<'v, 'msgs, 'msgsv, S> Scope<'v, 'msgs, 'msgsv, S> {
    pub fn new(
        variables: Option<&'v HashMap<String, VariableType>>,
        messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<S>>>,
    ) -> Self {
        Self {
            variables,
            messages,
        }
    }
}

trait MessagePartCollector<'s> {
    fn push_part(&mut self, part: MessagePart<'s>);
}

struct MessagePartsList<'s>(Vec<MessagePart<'s>>);
// struct MessageString<'s>(Cow<'s, str>);
// struct MessageSink<W>(W);

impl<'s> MessagePartsList<'s> {
    pub fn new() -> Self {
        Self(vec![])
    }
}

// impl<'s> MessageString<'s> {
//     pub fn new() -> Self {
//         Self("".into())
//     }
// }
//
// impl<W> MessageSink<W> {
//     pub fn new(sink: W) -> Self {
//         Self(sink)
//     }
// }

impl<'s> MessagePartCollector<'s> for MessagePartsList<'s> {
    fn push_part(&mut self, part: MessagePart<'s>) {
        self.0.push(part);
    }
}

// impl<'s, S: 's + Slice<'s>> MessagePartCollector<'s, S> for MessageString<'s> {
//     fn push_part(&mut self, part: MessagePart<'s, S>) {
//         if self.0.is_empty() {
//             match part {
//                 MessagePart::Literal(l) => self.0 = Cow::Borrowed(l.as_str()),
//                 MessagePart::Markup { name } => todo!(),
//                 MessagePart::MarkupEnd { name } => todo!(),
//                 MessagePart::PhantomData(_) => todo!(),
//             }
//         } else {
//             let new_part = match part {
//                 MessagePart::Literal(l) => Cow::Borrowed(l.as_str()),
//                 MessagePart::Markup { name } => todo!(),
//                 MessagePart::MarkupEnd { name } => todo!(),
//                 MessagePart::PhantomData(_) => todo!(),
//             };
//             if !new_part.is_empty() {
//                 self.0.to_mut().push_str(&new_part);
//             }
//         }
//     }
// }

// impl<'s, S: 's + Slice<'s>, W: std::fmt::Write> MessagePartCollector<'s, S> for MessageSink<W> {
//     fn push_part(&mut self, part: MessagePart<'s, S>) {
//         match part {
//             MessagePart::Literal(l) => self.0.write_str(l.as_str()).unwrap(),
//             MessagePart::Markup { name } => todo!(),
//             MessagePart::MarkupEnd { name } => todo!(),
//             MessagePart::PhantomData(_) => todo!(),
//         }
//     }
// }

pub struct Resolver {}

fn resolve_to_parts<'m, 'v, 'mv, 'h, 'mv2, 'p>(
    msg: &'m ast::PatternElement<&'mv str>,
    vars: Option<&'v VariableType>,
    msg2: Option<&'h ast::PatternElement<&'mv2 str>>,
) -> Vec<MessagePart<'p>>
where
    'm: 'p,
    'mv: 'p,
    'h: 'p,
    'v: 'p,
    'mv2: 'p,
{
    let s: &'mv str = match msg {
        ast::PatternElement::Text(s) => *s,
        ast::PatternElement::Placeholder(_) => todo!(),
    };
    let part: MessagePart<'p> = MessagePart::Literal(s);
    let mut result = vec![part];
    if let Some(msg2) = msg2 {
        let parts2 = resolve_to_parts(msg2, vars, None);
        result.extend(parts2);
        match vars.unwrap() {
            VariableType::String(s) => {
                result.push(MessagePart::Literal(s.as_str()));
            }
            _ => todo!(),
        }
    }
    result
}

#[cfg(test)]
mod test2 {
    use super::super::parser::Parser;
    use super::super::types::{MessagePart, VariableType};
    use super::ast;
    use super::{resolve_to_parts, Scope};
    use smallvec::SmallVec;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn test_sanity_check() {
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        // let mut variables = HashMap::new();
        // variables.insert("name".into(), VariableType::String("John".into()));
        // let scope = Scope::new(Some(&variables), None);
        let pat = ast::PatternElement::Text("Foo");
        let pat2 = ast::PatternElement::Text("Bar");
        let var = VariableType::String("Baz".into());
        let parts = resolve_to_parts(&pat, Some(&var), Some(&pat2));

        assert_eq!(
            parts,
            vec![
                MessagePart::Literal("Foo"),
                MessagePart::Literal("Bar"),
                MessagePart::Literal("Baz"),
            ]
        );
    }
}

// 'm - lifetime of the main message
// 'mv - lifetime of the value of the main message
// 's - lifetime of the scope
// 'v - lifetime of variables
// 'msgs - lifetime of messages
// 'msgsv - lifetime of messages value
// 'msgsc - lifetime of messages content
// 'p - lifetime of returned values
// S - generic of the main message
// S2 - generic of the referenced messages
impl<'m, 'mv, 's, 'v, 'msgs, 'msgsv, 'msgsc, 'p> Resolver {
    pub fn resolve_to_parts<S, S2>(
        msg: &'m ast::Message<S>,
        scope: &'s Scope<'v, 'msgs, 'msgsv, S2>,
    ) -> Vec<MessagePart<'p>>
    where
        S: Slice<'mv> + Clone + std::fmt::Debug,
        S2: Slice<'msgsc> + Clone + std::fmt::Debug,
        // 's: 'p,
        'mv: 'p,
        'm: 'p,
        // 'm: 'mv,
        // 's2: 'p,
        // 's3: 'p,
        // 'v: 'p,
        // 'h: 'p,
        // 'm: 'p,
        // 'h: 'p,
    {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        let mut collector = MessagePartsList::new();
        for part in &pattern.body {
            match part {
                ast::PatternElement::Text(s) => {
                    todo!()
                    // collector.push_part(MessagePart::Literal(s.as_str()));
                }
                ast::PatternElement::Placeholder(p) => {
                    todo!()
                    // match p {
                    //     ast::Placeholder::Expression(e) => {
                    //         match e {
                    //             ast::Expression::Operand { operand, annotation } => {
                    //                 match operand {
                    //                     ast::Operand::Literal(_) => {
                    //                         todo!();
                    //                     },
                    //                     ast::Operand::Variable(s) => {
                    //                         if let Some(v) = scope.variables.unwrap().get(s.as_str()) {
                    //                             match v {
                    //                                 VariableType::String(s) => {
                    //                                     collector.push_part(MessagePart::Literal(s.as_str()));
                    //                                 },
                    //                                 VariableType::MessageReference(s) => {
                    //                                     let m: &'m ast::Message<S2> = scope.messages.unwrap().get(s.as_str()).unwrap();
                    //                                     let parts = Self::resolve_to_parts(m, scope);
                    //                                     // for p in parts {
                    //                                     //     collector.push_part(p);
                    //                                     // }
                    //                                 },
                    //                                 _ => todo!(),
                    //                             }
                    //                         }
                    //                     },
                    //                 }
                    //             },
                    //             ast::Expression::Annotation(_) => todo!(),
                    //         }
                    //     },
                    //     _ => todo!(),
                    // }
                }
            }
        }
        collector.0
    }

    // pub fn resolve_to_string<S: 's + Slice<'s> + Clone + std::fmt::Debug>(
    //     msg: &ast::Message<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    // ) -> Cow<'s, str>
    // where
    //     'v: 's,
    // {
    //     let mut collector = MessageString::new();
    //     Self::resolve_message_to_collector(msg, scope, &mut collector);
    //     collector.0
    // }

    // pub fn resolve_to_sink<S: 's + Slice<'s> + Clone + std::fmt::Debug, W: std::fmt::Write>(
    //     msg: &ast::Message<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     sink: W,
    // ) where
    //     'v: 's,
    // {
    //     let mut collector = MessageSink::new(sink);
    //     Self::resolve_message_to_collector(msg, scope, &mut collector);
    // }

    // fn resolve_message_to_collector<
    //     S: 's + Slice<'s> + Clone + std::fmt::Debug,
    //     C: MessagePartCollector<'s, S>,
    // >(
    //     msg: &ast::Message<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     collector: &mut C,
    // ) where
    //     'v: 's,
    // {
    //     let value = &msg.value;
    //     let pattern = match value {
    //         ast::MessageValue::Pattern(pattern) => pattern,
    //         ast::MessageValue::Select(_) => todo!(),
    //     };
    //     let first_part = pattern.body.first().unwrap();
    //     let mp = match first_part {
    //         ast::PatternElement::Text(s) => MessagePart::Literal(s.clone()),
    //         ast::PatternElement::Placeholder(_) => todo!(),
    //     };
    //     collector.push_part(mp);
    //     // for pe in &pattern.body {
    //     //     Self::resolve_pattern_element(pe, scope, collector);
    //     // }
    // }

    // fn resolve_pattern_element<
    //     S: 's + Slice<'s> + Clone + std::fmt::Debug,
    //     C: MessagePartCollector<'s, S>,
    // >(
    //     pe: &ast::PatternElement<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     collector: &mut C,
    // ) where
    //     'v: 's,
    // {
    //     match pe {
    //         ast::PatternElement::Text(s) => collector.push_part(MessagePart::Literal(s.clone())),
    //         ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, scope, collector),
    //     }
    // }
    //
    // fn resolve_placeholder<
    //     S: 's + Slice<'s> + Clone + std::fmt::Debug,
    //     C: MessagePartCollector<'s, S>,
    // >(
    //     placeholder: &ast::Placeholder<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     collector: &mut C,
    // ) where
    //     'v: 's,
    // {
    //     match placeholder {
    //         ast::Placeholder::Markup { name, options } => todo!(),
    //         ast::Placeholder::MarkupEnd { name } => todo!(),
    //         ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope, collector),
    //     }
    // }
    //
    // fn resolve_expression<
    //     S: 's + Slice<'s> + Clone + std::fmt::Debug,
    //     C: MessagePartCollector<'s, S>,
    // >(
    //     exp: &ast::Expression<S>,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     collector: &mut C,
    // ) where
    //     'v: 's,
    // {
    //     match exp {
    //         ast::Expression::Operand {
    //             operand,
    //             annotation,
    //         } => match operand {
    //             ast::Operand::Literal(l) => {
    //                 collector.push_part(MessagePart::Literal(l.value.clone()))
    //             }
    //             ast::Operand::Variable(v) => Self::resolve_variable(v, scope, collector),
    //         },
    //         ast::Expression::Annotation(_) => todo!(),
    //     }
    // }
    //
    // fn resolve_variable<
    //     S: 's + Slice<'s> + Clone + std::fmt::Debug,
    //     C: MessagePartCollector<'s, S>,
    // >(
    //     variable: &S,
    //     scope: &Scope<'h, 'v, 'm, S>,
    //     collector: &mut C,
    // ) where
    //     'v: 's,
    // {
    //     if let Some(variables) = scope.variables {
    //         if let Some(v) = variables.get(variable.as_str()) {
    //             match v {
    //                 VariableType::String(s) => {
    //                     collector.push_part(MessagePart::Literal(S::from_str(s)))
    //                 }
    //                 VariableType::Number(_) => todo!(),
    //                 VariableType::MessageReference(id) => {
    //                     todo!()
    //                     // if let Some(messages) = scope.messages {
    //                     //     if let Some(msg) = messages.get(id.as_str()) {
    //                     //         //XXX: Optimize
    //                     //         let p = Self::resolve_to_parts(msg, scope);
    //                     //         parts.extend(p);
    //                     //     } else {
    //                     //         todo!()
    //                     //     }
    //                     // } else {
    //                     //     todo!()
    //                     // }
    //                 }
    //                 VariableType::Markup { name } => todo!(),
    //                 VariableType::MarkupEnd { name } => todo!(),
    //                 VariableType::List(_) => todo!(),
    //             }
    //         } else {
    //             todo!()
    //         }
    //     } else {
    //         todo!()
    //     }
    // }
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

    // #[test]
    // fn sanity_check() {
    //     let source = "{Hello World}";
    //     let parser = Parser::new(source);
    //     let msg = parser.parse().unwrap();
    //
    //     let mut variables = HashMap::new();
    //     variables.insert("name".into(), VariableType::String("John".into()));
    //     let scope = Scope::new(Some(&variables), None);
    //     let string = Resolver::resolve_to_string(&msg, &scope);
    //
    //     assert_eq!(string, "Hello World");
    // }

    // #[test]
    // fn stay_borrowed_check() {
    //     let parser = Parser::new("{Hello World{$name}{$creature}}");
    //     let msg = parser.parse().unwrap();
    //     // let msg = ast::Message {
    //     //     declarations: Default::default(),
    //     //     value: ast::MessageValue::Pattern(ast::Pattern {
    //     //         body: SmallVec::from_vec(vec![
    //     //             ast::PatternElement::Text("Hello World")
    //     //         ]),
    //     //     }),
    //     // };
    //
    //     // let scope = Scope::new(None, None);
    //     // let string = Resolver::resolve_to_string(&msg, &scope);
    //     //
    //     // assert_eq!(string, Cow::Borrowed("Hello World"));
    //
    //     // let scope = Scope::new(None, None);
    //     // let parts = Resolver::resolve_to_parts(&msg, &scope);
    //     //
    //     // assert_eq!(parts, vec![MessagePart::Literal("Hello World"),]);
    //     //
    //
    //     let parser = Parser::new("{Dragon}".to_string());
    //     let creature_msg = parser.parse().unwrap();
    //     let mut msgs = HashMap::new();
    //     msgs.insert("dragon".to_string(), &creature_msg);
    //
    //     let mut variables = HashMap::new();
    //     variables.insert("name".into(), VariableType::String("John".into()));
    //     variables.insert(
    //         "creature".into(),
    //         VariableType::MessageReference("dragon".into()),
    //     );
    //     let scope = Scope::new(Some(&variables), Some(&msgs));
    //     let parts = Resolver::resolve_to_parts(&msg, &scope);
    //
    //     assert_eq!(parts, vec![
    //         MessagePart::Literal("Hello World"),
    //         MessagePart::Literal("John"),
    //         MessagePart::Literal("Dragon"),
    //     ]);
    //     // let mut sink = String::new();
    //     // let scope = Scope::new(None, None);
    //     // Resolver::resolve_to_sink(&msg, &scope, &mut sink);
    //     //
    //     // assert_eq!(sink, "Hello World");
    // }
    //
    // #[test]
    // fn allocate_check() {
    //     let msg = ast::Message {
    //         declarations: Default::default(),
    //         value: ast::MessageValue::Pattern(ast::Pattern {
    //             body: SmallVec::from_vec(vec![
    //                 ast::PatternElement::Text("Hello "),
    //                 ast::PatternElement::Text("World"),
    //             ]),
    //         }),
    //     };
    //
    //     let scope = Scope::new(None, None);
    //     let string = Resolver::resolve_to_string(&msg, &scope);
    //
    //     assert_eq!(string, Cow::<str>::Owned(String::from("Hello World")));
    //
    //     let scope = Scope::new(None, None);
    //     let parts = Resolver::resolve_to_parts(&msg, &scope);
    //
    //     assert_eq!(
    //         parts,
    //         vec![
    //             MessagePart::Literal("Hello "),
    //             MessagePart::Literal("World"),
    //         ]
    //     );
    // }
    //
    // #[test]
    // fn variable_check() {
    //     let source = "{{$name}}";
    //     let parser = Parser::new(source);
    //     let msg = parser.parse().unwrap();
    //
    //     let mut variables = HashMap::new();
    //     variables.insert("name".into(), VariableType::String("John".into()));
    //     let scope = Scope::new(Some(&variables), None);
    //     let string = Resolver::resolve_to_string(&msg, &scope);
    //
    //     assert_eq!(string, "John");
    // }

    // #[test]
    // fn ref_msg_check() {
    //     let parser = Parser::new("{Dragon}");
    //     let dragon_msg = parser.parse().unwrap();
    //
    //     let parser = Parser::new("{Golem}");
    //     let golem_msg = parser.parse().unwrap();
    //
    //     let source = "{{$monster} killed you.}";
    //     let parser = Parser::new(source);
    //     let msg = parser.parse().unwrap();
    //
    //     let mut msgs = HashMap::new();
    //     msgs.insert("creature-dragon".to_string(), &dragon_msg);
    //     msgs.insert("creature-golem".to_string(), &golem_msg);
    //
    //     let mut variables = HashMap::new();
    //     variables.insert(
    //         "monster".into(),
    //         VariableType::MessageReference("creature-dragon".into()),
    //     );
    //
    //     let scope = Scope::new(Some(&variables), Some(&msgs));
    //     let string = Resolver::resolve_to_string(&msg, &scope);
    //
    //     assert_eq!(string, "Dragon killed you.");
    // }
}
