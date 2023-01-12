pub mod slice;
#[macro_use]
mod macros;

use super::ast;
use slice::Slice;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    Unknown,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::error::Error for ParserError {}

type ParserResult<R> = Result<R, ParserError>;

pub struct Parser<S> {
    source: S,
    ptr: usize,
}

impl<'s, S> Parser<S>
where
    S: Slice<'s>,
{
    #[inline]
    fn next_if(&mut self, b: u8) -> bool {
        let result = get_current_byte!(self) == Some(&b);
        if result {
            self.ptr += 1;
        }
        result
    }

    #[inline]
    fn next(&mut self) -> Option<&u8> {
        let result = get_current_byte!(self);
        self.ptr += 1;
        result
    }

    #[inline]
    fn skip_ws(&mut self) {
        while get_current_byte!(self) == Some(&b' ') {
            self.ptr += 1;
        }
    }
}

impl<'s, S> Parser<S>
where
    S: Slice<'s>,
{
    #[must_use]
    pub const fn new(source: S) -> Self {
        Self { source, ptr: 0 }
    }

    pub fn parse(mut self) -> ParserResult<ast::Message<S>> {
        let mut declarations = SmallVec::new();

        loop {
            match self.next() {
                Some(&b'l') => {
                    declarations.push(self.parse_declaration()?);
                    self.skip_ws();
                }
                Some(b) => {
                    let value = match b {
                        b'{' => {
                            let pattern = self.parse_pattern()?;
                            ast::MessageValue::Pattern(pattern)
                        }
                        b'm' => {
                            let select = self.parse_select()?;
                            ast::MessageValue::Select(Box::new(select))
                        }
                        _ => {
                            unreachable!();
                        }
                    };
                    return Ok(ast::Message {
                        declarations,
                        value,
                    });
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    fn parse_declaration(&mut self) -> ParserResult<ast::Declaration<S>> {
        assert_eq!(self.next(), Some(&b'e'));
        assert_eq!(self.next(), Some(&b't'));
        self.skip_ws();

        assert_eq!(self.next(), Some(&b'$'));
        let variable = self.parse_name()?;

        self.skip_ws();
        assert_eq!(self.next(), Some(&b'='));
        self.skip_ws();
        assert_eq!(self.next(), Some(&b'{'));
        let expression = self.parse_expression()?;
        assert_eq!(self.next(), Some(&b'}'));

        Ok(ast::Declaration {
            variable,
            expression,
        })
    }

    fn parse_select(&mut self) -> ParserResult<ast::Select<S>> {
        assert_eq!(self.next(), Some(&b'a'));
        assert_eq!(self.next(), Some(&b't'));
        assert_eq!(self.next(), Some(&b'c'));
        assert_eq!(self.next(), Some(&b'h'));
        let mut selector = SmallVec::new();
        let mut variants = SmallVec::new();

        self.skip_ws();

        while self.next_if(b'{') {
            selector.push(self.parse_expression()?);
            assert_eq!(self.next(), Some(&b'}'));
            self.skip_ws();
        }

        while self.next_if(b'w') {
            variants.push(self.parse_variant()?);
            self.skip_ws();
        }

        Ok(ast::Select { selector, variants })
    }

    fn parse_variant(&mut self) -> ParserResult<ast::Variant<S>> {
        assert_eq!(self.next(), Some(&b'h'));
        assert_eq!(self.next(), Some(&b'e'));
        assert_eq!(self.next(), Some(&b'n'));
        let mut key = SmallVec::new();

        self.skip_ws();

        if self.next_if(b'*') {
            key.push(ast::VariantKey::Asterisk);
        }

        self.skip_ws();

        assert_eq!(self.next(), Some(&b'{'));

        let pattern = self.parse_pattern()?;

        Ok(ast::Variant { key, pattern })
    }

    fn parse_pattern(&mut self) -> ParserResult<ast::Pattern<S>> {
        let mut start = self.ptr;
        let mut body = SmallVec::new();
        while let Some(b) = self.next() {
            match b {
                b'}' => {
                    let end = self.ptr - 1;
                    if start != end {
                        body.push(ast::PatternElement::Text(self.source.slice(start..end)));
                    }
                    return Ok(ast::Pattern { body });
                }
                b'{' => {
                    let end = self.ptr - 1;
                    if start != end {
                        body.push(ast::PatternElement::Text(self.source.slice(start..end)));
                    }
                    body.push(ast::PatternElement::Placeholder(self.parse_placeholder()?));
                    start = self.ptr;
                }
                _ => {}
            }
        }
        unreachable!()
    }

    fn parse_placeholder(&mut self) -> ParserResult<ast::Placeholder<S>> {
        let placeholder = match get_current_byte!(self) {
            Some(b'+') => {
                self.ptr += 1;
                let name = self.parse_name()?;
                let options = SmallVec::new();
                ast::Placeholder::Markup { name, options }
            }
            Some(b'-') => {
                self.ptr += 1;
                let name = self.parse_name()?;
                ast::Placeholder::MarkupEnd { name }
            }
            Some(_) => {
                let exp = self.parse_expression()?;
                ast::Placeholder::Expression(exp)
            }
            None => {
                unreachable!();
            }
        };
        assert_eq!(self.next(), Some(&b'}'));
        Ok(placeholder)
    }

    fn parse_expression(&mut self) -> ParserResult<ast::Expression<S>> {
        let operand = self.parse_operand()?;
        let annotation = if self.next_if(b' ') {
            Some(self.parse_annotation()?)
        } else {
            None
        };
        Ok(ast::Expression::Operand {
            operand,
            annotation,
        })
    }

    fn parse_operand(&mut self) -> ParserResult<ast::Operand<S>> {
        let op = match self.next() {
            Some(b'$') => ast::Operand::Variable(self.parse_name()?),
            Some(b'(') => ast::Operand::Literal(self.parse_literal()?),
            _ => {
                unreachable!()
            }
        };
        Ok(op)
    }

    fn parse_annotation(&mut self) -> ParserResult<ast::Annotation<S>> {
        assert_eq!(self.next(), Some(&b':'));
        let name = self.parse_name()?;
        Ok(ast::Annotation {
            function: name,
            options: SmallVec::new(),
        })
    }

    fn parse_name(&mut self) -> ParserResult<S> {
        let start = self.ptr;
        if let Some(ch) = self.next() {
            assert!(ch.is_ascii_alphabetic());
        } else {
            unreachable!();
        }

        while let Some(b) = get_current_byte!(self) {
            if b.is_ascii_alphabetic() || *b == b'-' {
                self.ptr += 1;
            } else {
                break;
            }
        }
        if start == self.ptr {
            unreachable!();
        } else {
            Ok(self.source.slice(start..self.ptr))
        }
    }

    fn parse_literal(&mut self) -> ParserResult<ast::Literal<S>> {
        let start = self.ptr;
        while let Some(b) = self.next() {
            if b == &b')' {
                break;
            }
        }

        if start == self.ptr - 1 {
            unreachable!();
        } else {
            Ok(ast::Literal {
                value: self.source.slice(start..self.ptr - 1),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ast;
    use super::Parser;
    use smallvec::SmallVec;

    #[test]
    fn test_message() {
        let source = "{Hello World}";
        let parser = Parser::new(source);

        let ast = parser.parse();
        assert_eq!(
            ast,
            Ok(ast::Message {
                declarations: SmallVec::new(),
                value: ast::MessageValue::Pattern(ast::Pattern {
                    body: SmallVec::from_vec(vec![ast::PatternElement::Text("Hello World")])
                })
            })
        );
    }

    #[test]
    fn test_placeholder() {
        let source = "{Today is {$today} a good day.}";
        let parser = Parser::new(source);

        let ast = parser.parse();
        assert_eq!(
            ast,
            Ok(ast::Message {
                declarations: SmallVec::new(),
                value: ast::MessageValue::Pattern(ast::Pattern {
                    body: SmallVec::from_vec(vec![
                        ast::PatternElement::Text("Today is "),
                        ast::PatternElement::Placeholder(ast::Placeholder::Expression(
                            ast::Expression::Operand {
                                operand: ast::Operand::Variable("today"),
                                annotation: None,
                            }
                        )),
                        ast::PatternElement::Text(" a good day."),
                    ])
                })
            })
        );
    }

    #[test]
    fn test_literal() {
        let source = "{Today is {(This is a Literal)}}";
        let parser = Parser::new(source);

        let ast = parser.parse();
        assert_eq!(
            ast,
            Ok(ast::Message {
                declarations: SmallVec::new(),
                value: ast::MessageValue::Pattern(ast::Pattern {
                    body: SmallVec::from_vec(vec![
                        ast::PatternElement::Text("Today is "),
                        ast::PatternElement::Placeholder(ast::Placeholder::Expression(
                            ast::Expression::Operand {
                                operand: ast::Operand::Literal(ast::Literal {
                                    value: "This is a Literal"
                                }),
                                annotation: None,
                            }
                        )),
                    ])
                })
            })
        );
    }

    #[test]
    fn test_select() {
        let source = "match {$var} when * {Zero}";
        let parser = Parser::new(source);

        let ast = parser.parse();
        assert_eq!(
            ast,
            Ok(ast::Message {
                declarations: SmallVec::new(),
                value: ast::MessageValue::Select(Box::new(ast::Select {
                    selector: SmallVec::from_vec(vec![ast::Expression::Operand {
                        operand: ast::Operand::Variable("var"),
                        annotation: None,
                    }]),
                    variants: SmallVec::from_vec(vec![ast::Variant {
                        key: SmallVec::from_vec(vec![ast::VariantKey::Asterisk,]),
                        pattern: ast::Pattern {
                            body: SmallVec::from_vec(vec![ast::PatternElement::Text("Zero"),]),
                        }
                    }]),
                }))
            })
        );
    }

    #[test]
    fn test_declarations() {
        let source = "let $foo = {$bar} {Welcome to {$foo}}";
        let parser = Parser::new(source);

        let ast = parser.parse();
        assert_eq!(
            ast,
            Ok(ast::Message {
                declarations: SmallVec::from_vec(vec![ast::Declaration {
                    variable: "foo",
                    expression: ast::Expression::Operand {
                        operand: ast::Operand::Variable("bar"),
                        annotation: None,
                    },
                },]),
                value: ast::MessageValue::Pattern(ast::Pattern {
                    body: SmallVec::from_vec(vec![
                        ast::PatternElement::Text("Welcome to "),
                        ast::PatternElement::Placeholder(ast::Placeholder::Expression(
                            ast::Expression::Operand {
                                operand: ast::Operand::Variable("foo"),
                                annotation: None,
                            },
                        )),
                    ])
                })
            })
        );
    }
}
