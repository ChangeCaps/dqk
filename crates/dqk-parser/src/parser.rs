use std::{iter::Peekable, path::Path, str::Chars};

use crate::{
    Error, Integer, IntegerKind, Keyword, Parse, Result, Span, StringAllocator, Symbol, Token,
    TokenKind,
};

pub struct Parser<'a> {
    string_allocator: &'a mut StringAllocator,
    file_path: &'static Path,
    start: usize,
    chars: Peekable<Chars<'a>>,
    next_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(
        src: &'a str,
        file_path: &'static Path,
        string_allocator: &'a mut StringAllocator,
    ) -> Self {
        Self {
            string_allocator,
            file_path,
            start: 0,
            chars: src.chars().peekable(),
            next_token: None,
        }
    }

    pub fn span(&self) -> Span {
        Span::new(self.file_path, self.start, 0)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.next();

        if ch.is_some() {
            self.start += 1;
        }

        ch
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek_char()
            .map_or(false, |c| char::is_whitespace(c) && c != '\n')
        {
            self.next_char();
        }
    }

    fn parse_integer(&mut self) -> Option<Result<Integer>> {
        let mut value = 0i64;
        let mut digits = 0;
        let mut kind = IntegerKind::Decimal;

        loop {
            if let Some(ch) = self.peek_char() {
                if let Some(digit) = ch.to_digit(kind.radix()) {
                    value += digit as i64 * (kind.radix() as i64).pow(digits);
                    digits += 1;

                    self.next_char();
                    continue;
                } else if ch == 'b' && value == 0 && digits == 1 && kind == IntegerKind::Decimal {
                    digits = 0;
                    kind = IntegerKind::Binary;

                    self.next_char();
                    continue;
                } else if ch == 'x' && value == 0 && digits == 1 && kind == IntegerKind::Decimal {
                    digits = 0;
                    kind = IntegerKind::Hex;

                    self.next_char();
                    continue;
                }
            }

            if digits == 0 && kind != IntegerKind::Decimal {
                let error = Error::new("malformed integer")
                    .with_hint("expected integer after radix", self.span());

                break Some(Err(error));
            } else if digits > 0 {
                break Some(Ok(Integer::new(value, kind)));
            } else {
                break None;
            }
        }
    }

    fn parse_ident(&mut self) -> Option<&'static str> {
        let mut ident = String::new();

        loop {
            if let Some(ch) = self.peek_char() {
                if ch == '_' || ch.is_alphabetic() || (!ident.is_empty() && ch.is_numeric()) {
                    ident.push(ch);
                    self.next_char();

                    continue;
                }
            }

            if ident.is_empty() {
                break None;
            } else {
                break Some(self.string_allocator.get(&ident));
            }
        }
    }

    fn parse_symbol(&mut self) -> Option<Symbol> {
        macro_rules! symbol {
            ($($ch:literal => $(> $second:literal => $second_symbol:expr,)* $symbol:expr $(,)?,)*) => {
                match self.peek_char() {
					$(Some($ch) => {
                        self.next_char();

						match self.peek_char() {
                            $(Some($second) => {
                                self.next_char();

                                Some($second_symbol)
                            })*
                            _ => Some($symbol)
                        }
					})*
                    _ => None,
                }
            };
        }

        use Symbol::*;

        symbol! {
            '(' => OpenParen,
            '{' => OpenBrace,
            '[' => OpenBracket,
            ')' => CloseParen,
            '}' => CloseBrace,
            ']' => CloseBracket,
            ':' =>
                > '=' => ColonEqual,
                Colon,
            ';' => SemiColon,
            ',' => Comma,
            '.' => Period,
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '=' =>
                > '=' => EqualEqual,
                Equal,
            '>' =>
                > '=' => GtEqual,
                Gt,
            '<' =>
                > '=' => LtEqual,
                Lt,
        }
    }

    fn parse_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        if self.peek_char().is_none() {
            return Ok(Token::new(TokenKind::Eof, self.span()));
        }

        if self.peek_char() == Some('\n') {
            self.next_char();
            return Ok(Token::new(TokenKind::Eol, self.span()));
        }

        let start = self.span();

        if let Some(integer) = self.parse_integer() {
            return Ok(Token::new(
                TokenKind::Integer(integer?),
                start | self.span(),
            ));
        }

        if let Some(ident) = self.parse_ident() {
            let kind = match ident {
                "event" => TokenKind::Keyword(Keyword::Event),
                "query" => TokenKind::Keyword(Keyword::Query),
                "where" => TokenKind::Keyword(Keyword::Where),
                "ln" => TokenKind::Keyword(Keyword::Ln),
                "fn" => TokenKind::Keyword(Keyword::Fn),
                "if" => TokenKind::Keyword(Keyword::If),
                "for" => TokenKind::Keyword(Keyword::For),
                "else" => TokenKind::Keyword(Keyword::Else),
                "return" => TokenKind::Keyword(Keyword::Return),
                _ => TokenKind::Ident(ident),
            };
            return Ok(Token::new(kind, start | self.span()));
        }

        if let Some(symbol) = self.parse_symbol() {
            return Ok(Token::new(TokenKind::Symbol(symbol), start | self.span()));
        }

        let msg = format!("found character '{}'", self.next_char().unwrap());
        Err(Error::new("unexpected character").with_hint(msg, self.span()))
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if let Some(token) = self.next_token.take() {
            Ok(token)
        } else {
            self.parse_token()
        }
    }

    pub fn peek_token(&mut self) -> Result<Token> {
        if let Some(token) = self.next_token.clone().take() {
            Ok(token)
        } else {
            let token = self.parse_token()?;
            self.next_token = Some(token.clone());
            Ok(token)
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Span> {
        let tok = self.next_token()?;

        if tok.kind() == kind {
            Ok(tok.span())
        } else {
            Err(Error::new(format!("expected '{:?}'", kind))
                .with_hint(format!("found '{:?}'", tok.kind()), tok.span()))
        }
    }

    pub fn expect_eol(&mut self) -> Result<Span> {
        let tok = self.next_token()?;

        match tok.kind() {
            TokenKind::Eol | TokenKind::Eof => Ok(tok.span()),
            kind => Err(Error::new("expected 'end of line'")
                .with_hint(format!("found '{:?}'", kind), tok.span())),
        }
    }

    pub fn skip_eol(&mut self) -> Result<()> {
        while self.peek_token()?.kind() == TokenKind::Eol {
            self.next_token()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
