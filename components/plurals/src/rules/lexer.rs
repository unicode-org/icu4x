// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::ast;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operand(ast::Operand),
    Operator(ast::Operator),
    Number(u32),
    Zero,
    Dot,
    DotDot,
    Comma,
    Or,
    And,
    Modulo,
    Junk,
    At,
    Decimal,
    Integer,
    Ellipsis,
    Tilde,
}

#[derive(Debug)]
pub enum LexerError {
    ExpectedByte(u8),
    UnknownToken(u8),
}

/// Unicode Plural Rule lexer is an iterator
/// over tokens produced from an input string.
///
/// # Examples
///
/// ```
/// use icu_plurals::rules::Lexer;
///
/// let input = b"i = 5";
/// let lexer = Lexer::new(input);
/// assert_eq!(lexer.count(), 3);
/// ```
pub struct Lexer<'l> {
    chars: &'l [u8],
    ptr: usize,
}

impl<'l> Lexer<'l> {
    /// Constructs a new `Lexer` for a given input.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_plurals::rules::Lexer;
    ///
    /// Lexer::new(b"n = 1");
    /// ```
    pub fn new(input: &'l [u8]) -> Self {
        Self {
            chars: input,
            ptr: 0,
        }
    }

    fn bump(&mut self) -> Option<&u8> {
        let ret = self.chars.get(self.ptr);
        self.ptr += 1;
        ret
    }

    fn take_if(&mut self, c: u8) -> bool {
        if self.chars.get(self.ptr) == Some(&&c) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), LexerError> {
        if self.bump() == Some(&expected) {
            Ok(())
        } else {
            Err(LexerError::ExpectedByte(expected))
        }
    }

    fn advance_token(&mut self) -> Result<Option<Token>, LexerError> {
        loop {
            if let Some(c) = self.bump() {
                let token = match c {
                    b' ' => continue,
                    b'n' => Token::Operand(ast::Operand::N),
                    b'i' => {
                        if self.take_if(b'n') {
                            self.expect(b't')?;
                            self.expect(b'e')?;
                            self.expect(b'g')?;
                            self.expect(b'e')?;
                            self.expect(b'r')?;
                            Token::Integer
                        } else {
                            Token::Operand(ast::Operand::I)
                        }
                    }
                    b'f' => Token::Operand(ast::Operand::F),
                    b't' => Token::Operand(ast::Operand::T),
                    b'v' => Token::Operand(ast::Operand::V),
                    b'w' => Token::Operand(ast::Operand::W),
                    b'=' => Token::Operator(ast::Operator::Eq),
                    // Zero is special, because we need to preserve it for Samples.
                    b'0' => Token::Zero,
                    b'1'..=b'9' => {
                        let start = self.ptr - 1;

                        while let Some(b'0'..=b'9') = self.chars.get(self.ptr) {
                            self.ptr += 1;
                        }
                        let end = self.ptr;

                        let mut value = 0;
                        for ptr in start..end {
                            let mul = 10u32.pow((end - ptr - 1) as u32);
                            value += ((self.chars[ptr] - b'0') as u32) * mul;
                        }
                        Token::Number(value)
                    }
                    b'a' => {
                        self.expect(b'n')?;
                        self.expect(b'd')?;
                        Token::And
                    }
                    b'o' => {
                        self.expect(b'r')?;
                        Token::Or
                    }
                    b'!' => {
                        self.expect(b'=')?;
                        Token::Operator(ast::Operator::NotEq)
                    }
                    b'.' => {
                        if self.take_if(b'.') {
                            Token::DotDot
                        } else {
                            Token::Dot
                        }
                    }
                    b'd' => {
                        self.expect(b'e')?;
                        self.expect(b'c')?;
                        self.expect(b'i')?;
                        self.expect(b'm')?;
                        self.expect(b'a')?;
                        self.expect(b'l')?;
                        Token::Decimal
                    }
                    b',' => Token::Comma,
                    b'%' => Token::Modulo,
                    b'@' => Token::At,
                    226 => {
                        // Ellipsis
                        self.expect(128)?;
                        self.expect(166)?;
                        Token::Ellipsis
                    }
                    b'~' => Token::Tilde,
                    b => return Err(LexerError::UnknownToken(*b)),
                };
                return Ok(Some(token));
            } else {
                return Ok(None);
            }
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.advance_token().unwrap_or(Some(Token::Junk))
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
