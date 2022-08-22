pub mod ast;
pub mod functions;
pub mod parser;
pub mod resolver;
pub mod types;

use icu_locid::Locale;
use intl_memoizer::IntlMemoizer;
use parser::{slice::Slice, Parser};
use resolver::{Resolver, Scope};
use std::borrow::Cow;
use std::collections::HashMap;
use types::{MessagePart, VariableType};

pub type MF2Function<'b> =
    Box<dyn for<'s> Fn(&VariableType<&'s str>, &MessageFormat) -> Vec<MessagePart<String>> + 'b>;

#[derive(Default)]
pub struct MessageFormat<'b> {
    pub intls: IntlMemoizer,
    pub functions: HashMap<String, MF2Function<'b>>,
}

impl<'b> MessageFormat<'b> {
    pub fn new(_locale: Locale) -> Self {
        Self {
            intls: IntlMemoizer::default(),
            functions: HashMap::default(),
        }
    }

    pub fn format_to_string<'m, 'mv, 'varsv, 'varsm, 'mf, 'mpv, MV, VARSV>(
        &'mf self,
        msg: &'m ast::Message<MV>,
        variables: Option<&'varsm HashMap<String, VariableType<VARSV>>>,
    ) -> Cow<'mpv, str>
    where
        MV: Slice<'mv>,
        VARSV: Slice<'varsv>,
        'mv: 'mpv,
        'varsv: 'mpv,
        'varsm: 'varsv,
    {
        let scope = Scope::new(self, variables);
        Resolver::<_, _, Cow<str>>::resolve_to_string(msg, &scope)
    }

    pub fn format_to_parts<'m, 'mv, 'varsv, 'varsm, 'mf, 'mpv, MV, VARSV, MPV>(
        &self,
        msg: &ast::Message<MV>,
        variables: Option<&'varsm HashMap<String, VariableType<VARSV>>>,
    ) -> Vec<MessagePart<MPV>>
    where
        MV: Slice<'mv>,
        VARSV: Slice<'varsv>,
        MPV: 'mpv + Slice<'mpv>,
        'mv: 'mpv,
        'varsv: 'mpv,
        'varsm: 'varsv,
    {
        let scope = Scope::new(self, variables);
        Resolver::resolve_to_parts(msg, &scope)
    }

    pub fn format_from_source<'m, 'mv, 'varsv, 'varsm, 'mf, 'mpv, MV, VARSV>(
        &'mf self,
        source: MV,
        variables: Option<&'varsm HashMap<String, VariableType<VARSV>>>,
    ) -> Cow<'mpv, str>
    where
        MV: 'm + Slice<'mv>,
        VARSV: Slice<'varsv>,
        'mv: 'mpv,
        'varsv: 'mpv,
        'varsm: 'varsv,
    {
        let parser = Parser::new(source);
        let msg: ast::Message<MV> = parser.parse().unwrap();
        self.format_to_string(&msg, variables)
    }
}

#[cfg(test)]
mod test {
    use super::parser::Parser;
    use super::types::{MessagePart, VariableType};
    use super::MessageFormat;
    use crate::ast;
    use icu_locid::locale;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let mf = MessageFormat::new(locale!("und"));

        let result = mf.format_from_source::<_, &str>("{Hello World}", None);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn variable_check() {
        let mf = MessageFormat::new(locale!("und"));

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));

        let result = mf.format_from_source("{{$name}}", Some(&variables));
        assert_eq!(result, "John");
    }

    #[test]
    fn function_check() {
        let mut mf = MessageFormat::new(locale!("und"));
        mf.functions.insert(
            "number".to_string(),
            Box::new(
                |input: &VariableType<&str>, mf: &MessageFormat| -> Vec<MessagePart<String>> {
                    match input {
                        VariableType::Number(n) => {
                            let result = format!("{n}");
                            vec![MessagePart::Literal(result)]
                        }
                        _ => todo!(),
                    }
                },
            ),
        );

        let mut variables: HashMap<_, VariableType<&str>> = HashMap::new();
        variables.insert("emailCount".into(), VariableType::Number(5.0));

        let result = mf.format_from_source(
            "{You have {$emailCount :number} unread emails.}",
            Some(&variables),
        );
        assert_eq!(result, "You have 5 unread emails.");
    }

    #[test]
    fn dynamic_msg_check() {
        let mut messages = HashMap::new();

        let parser = Parser::new("{Dragon}");
        let dragon_msg = parser.parse().unwrap();
        let parser = Parser::new("{Golem}");
        let golem_msg = parser.parse().unwrap();

        messages.insert("creature-dragon".to_string(), &dragon_msg);
        messages.insert("creature-golem".to_string(), &golem_msg);

        let msg_ref = &messages;

        let mut mf = MessageFormat::new(locale!("und"));

        let message_function =
            |input: &VariableType<&str>, mf: &MessageFormat| -> Vec<MessagePart<String>> {
                let id: &str = match input {
                    VariableType::MessageReference(s) => *s,
                    _ => todo!(),
                };
                let msg = msg_ref.get(id).unwrap();
                let result = mf.format_to_string::<_, &str>(msg, None);
                vec![MessagePart::Literal(result.to_string())]
            };

        mf.functions
            .insert("message".to_string(), Box::new(message_function));

        let mut variables = HashMap::new();
        variables.insert(
            "monster".into(),
            VariableType::MessageReference("creature-dragon"),
        );

        let result = mf.format_from_source("{{$monster :message} killed you.}", Some(&variables));
        assert_eq!(result, "Dragon killed you.");
    }

    #[test]
    fn function_preserve_parts() {
        let mut mf = MessageFormat::new(locale!("und"));
        mf.functions.insert(
            "emphasis".to_string(),
            Box::new(
                |input: &VariableType<&str>, mf: &MessageFormat| -> Vec<MessagePart<String>> {
                    let v = match input {
                        VariableType::String(s) => s,
                        _ => todo!(),
                    };
                    vec![
                        MessagePart::Markup {
                            name: "strong".to_string(),
                        },
                        MessagePart::Literal(v.to_string()),
                        MessagePart::MarkupEnd {
                            name: "strong".to_string(),
                        },
                    ]
                },
            ),
        );

        let mut variables = HashMap::new();
        variables.insert("userName".into(), VariableType::String("John"));

        let result = mf.format_from_source("{Hello {$userName :emphasis}.}", Some(&variables));
        assert_eq!(result, "Hello {+strong}John{-strong}.");
    }

    #[test]
    fn markup_passthrough_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let mut variables = HashMap::new();
        variables.insert(
            "input-markup".into(),
            VariableType::List(vec![
                VariableType::Markup { name: "strong" },
                VariableType::String("Hello World!"),
                VariableType::MarkupEnd { name: "strong" },
            ]),
        );

        let parser = Parser::new("{{$input-markup}}");
        let msg = parser.parse().unwrap();

        let result = mf.format_to_parts(&msg, Some(&variables));
        assert_eq!(
            result,
            vec![
                MessagePart::Markup {
                    name: Cow::Borrowed("strong")
                },
                MessagePart::Literal(Cow::Borrowed("Hello World!")),
                MessagePart::MarkupEnd {
                    name: Cow::Borrowed("strong")
                },
            ]
        );
    }
}
