pub mod ast;
pub mod parser;
pub mod resolver;
pub mod types;
use parser::{slice::Slice, Parser};
use resolver::{Resolver, Scope};
use std::borrow::Cow;
use std::collections::HashMap;
use types::{MessagePart, VariableType};

#[derive(Default)]
pub struct MessageFormat<'msgsmv, MSGSV> {
    pub messages: HashMap<String, &'msgsmv ast::Message<MSGSV>>,
}

impl<'msgsmv, MSGSV> MessageFormat<'msgsmv, MSGSV> {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
        }
    }

    pub fn format_to_string<'m, 'mv, 'varsv, 'varsm, 'msgsm, 'msgsv, 'mpv, MV, VARSV>(
        &'msgsm self,
        msg: &'m ast::Message<MV>,
        variables: Option<&'varsm HashMap<String, VariableType<VARSV>>>,
    ) -> Cow<'mpv, str>
    where
        MV: Slice<'mv>,
        VARSV: Slice<'varsv>,
        MSGSV: Slice<'msgsv>,
        'mv: 'mpv,
        'msgsv: 'mpv,
        'varsv: 'mpv,
    {
        let scope = Scope::new(variables, Some(&self.messages));
        Resolver::<_, _, _, &str>::resolve_to_string(msg, &scope)
    }
    //
    // pub fn format_to_parts(
    //     &self,
    //     msg: &ast::Message<S>,
    //     variables: Option<HashMap<String, VariableType>>,
    // ) -> Vec<MessagePart<S>> {
    //     let scope = Scope::new(variables, Some(&self.messages));
    //     Resolver::resolve_to_parts(msg, &scope)
    // }

    pub fn format_from_source<'m, 'mv, 'varsv, 'varsm, 'msgsm, 'msgsv, 'mpv, MV, VARSV>(
        &'msgsm self,
        source: MV,
        variables: Option<&'varsm HashMap<String, VariableType<VARSV>>>,
    ) -> Cow<'mpv, str>
    where
        MV: 'm + Slice<'mv>,
        VARSV: Slice<'varsv>,
        MSGSV: Slice<'msgsv>,
        'mv: 'mpv,
        'msgsv: 'mpv,
        'varsv: 'mpv,
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
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let mf = MessageFormat::<&str>::new();

        let result = mf.format_from_source::<&str, &str>("{Hello World}", None);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn variable_check() {
        let mf = MessageFormat::<&str>::new();

        let mut variables = HashMap::new();
        variables.insert("name".into(), VariableType::String("John"));

        let result = mf.format_from_source("{{$name}}", Some(&variables));
        assert_eq!(result, "John");
    }

    #[test]
    fn ref_msg_check() {
        let mut mf = MessageFormat::new();

        let parser = Parser::new("{Dragon}");
        let dragon_msg = parser.parse().unwrap();
        let parser = Parser::new("{Golem}");
        let golem_msg = parser.parse().unwrap();

        mf.messages
            .insert("creature-dragon".to_string(), &dragon_msg);
        mf.messages.insert("creature-golem".to_string(), &golem_msg);

        let mut variables = HashMap::new();
        variables.insert(
            "monster".into(),
            VariableType::MessageReference("creature-dragon"),
        );

        let result = mf.format_from_source("{{$monster} killed you.}", Some(&variables));
        assert_eq!(result, "Dragon killed you.");
    }

    // #[test]
    // fn markup_passthrough_check() {
    //     let mf = MessageFormat::new();
    //
    //     let mut variables = HashMap::new();
    //     variables.insert(
    //         "input-markup".into(),
    //         VariableType::List(vec![
    //             VariableType::Markup {
    //                 name: "strong",
    //             },
    //             VariableType::String(String::from("Hello World!")),
    //             VariableType::MarkupEnd {
    //                 name: "strong",
    //             },
    //         ]),
    //     );
    //
    //     let parser = Parser::new("{{$input-markup}}");
    //     let msg = parser.parse().unwrap();
    //
    //     let result = mf.format_to_parts(&msg, Some(variables));
    //     assert_eq!(
    //         result,
    //         vec![
    //             MessagePart::Markup {
    //                 name: Cow::Borrowed("strong")
    //             },
    //             MessagePart::Literal(Cow::Borrowed("Hello World!")),
    //             MessagePart::MarkupEnd {
    //                 name: Cow::Borrowed("strong")
    //             },
    //         ]
    //     );
    // }
}
