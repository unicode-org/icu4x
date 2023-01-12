use crate::{parser::slice::Slice, types::MessagePart};
use std::borrow::Cow;

// MPV - message part value type
pub trait MessagePartCollector<MPV> {
    fn push_part(&mut self, part: MessagePart<MPV>);
}

pub struct MessagePartsList<MPV>(pub Vec<MessagePart<MPV>>);
pub struct MessageString<'s>(pub Cow<'s, str>);
pub struct MessageSink<W>(W);

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

impl<MPV> MessagePartCollector<MPV> for MessagePartsList<MPV> {
    fn push_part(&mut self, part: MessagePart<MPV>) {
        self.0.push(part);
    }
}

impl<'s, MPV: Slice<'s>> MessagePartCollector<MPV> for MessageString<'s> {
    fn push_part(&mut self, part: MessagePart<MPV>) {
        let new_part = match part {
            MessagePart::Literal(l) => l.into_cow(),
            MessagePart::Markup { name } => {
                let name = name.as_str();
                let result = format!("{{+{name}}}");
                result.into_cow()
            }
            MessagePart::MarkupEnd { name } => {
                let name = name.as_str();
                let result = format!("{{-{name}}}");
                result.into_cow()
            }
        };
        if !new_part.is_empty() {
            if self.0.is_empty() {
                self.0 = new_part;
            } else {
                self.0.to_mut().push_str(&new_part);
            }
        }
    }
}

impl<'s, MPV: 's + Slice<'s>, W: std::fmt::Write> MessagePartCollector<MPV> for MessageSink<W> {
    fn push_part(&mut self, part: MessagePart<MPV>) {
        match part {
            MessagePart::Literal(l) => self.0.write_str(l.as_str()).unwrap(),
            MessagePart::Markup { name } => {
                self.0.write_str("{{+").unwrap();
                self.0.write_str(name.as_str()).unwrap();
                self.0.write_str("}}").unwrap();
            }
            MessagePart::MarkupEnd { name } => {
                self.0.write_str("{{-").unwrap();
                self.0.write_str(name.as_str()).unwrap();
                self.0.write_str("}}").unwrap();
            }
        }
    }
}
