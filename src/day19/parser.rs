use crate::day19::models::{Category, Part, ReviewResult, Rule, System, Workflow};
use std::collections::HashMap;
use std::iter::Peekable;

pub(super) struct Lexer<I: Iterator> {
    input: Peekable<I>,
}
#[derive(Clone, PartialEq, Debug)]
pub(super) enum Token {
    Identifier(String),
    OpenBrace,
    CloseBrace,
    OpenAngle,
    CloseAngle,
    Number(u64),
    Comma,
    Colon,
    Accept,
    Reject,
    Coolness,
    Musical,
    Aerodynamic,
    Shiny,
    Equals,
    End,
}
impl<I: Iterator<Item = char>> Lexer<I> {
    pub(super) fn new(input: Peekable<I>) -> Self {
        Self { input }
    }
}

pub(super) struct LexerIntoIterator<I: Iterator> {
    input: Peekable<I>,
    end: bool,
}
impl<I: Iterator<Item = char>> IntoIterator for Lexer<I> {
    type Item = Token;
    type IntoIter = LexerIntoIterator<I>;
    fn into_iter(self) -> Self::IntoIter {
        LexerIntoIterator {
            input: self.input,
            end: false,
        }
    }
}
impl<I: Iterator<Item = char>> Iterator for LexerIntoIterator<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        while let Some(c) = self.input.next() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    match self.input.peek() {
                        Some(&c) if c.is_alphabetic() => (),
                        _ => match c {
                            'x' => return Some(Token::Coolness),
                            'm' => return Some(Token::Musical),
                            'a' => return Some(Token::Aerodynamic),
                            's' => return Some(Token::Shiny),
                            'A' => return Some(Token::Accept),
                            'R' => return Some(Token::Reject),
                            _ => (),
                        },
                    }
                    let mut s = String::new();
                    s.push(c);
                    while let Some(&c) = self.input.peek() {
                        if c.is_alphabetic() {
                            s.push(c);
                            self.input.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Identifier(s));
                }
                '{' => return Some(Token::OpenBrace),
                '}' => return Some(Token::CloseBrace),
                '<' => return Some(Token::OpenAngle),
                '>' => return Some(Token::CloseAngle),
                ',' => return Some(Token::Comma),
                ':' => return Some(Token::Colon),
                '=' => return Some(Token::Equals),
                '0'..='9' => {
                    let mut n = c.to_digit(10).unwrap() as u64;
                    while let Some(&c) = self.input.peek() {
                        if c.is_ascii_digit() {
                            n = n * 10 + c.to_digit(10).unwrap() as u64;
                            self.input.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Number(n));
                }
                _ if c.is_whitespace() => continue,
                _ => break,
            }
        }

        self.end = true;
        Some(Token::End)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum ParserError {
    ExpectedCategory,
    UnexpectedEnd,
    ExpectedToken(Token),
    ExpectedRule,
}
type Result<T> = std::result::Result<T, ParserError>;
pub(super) struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}
impl<I: Iterator<Item = Token>> Parser<I> {
    pub(super) fn new(tokens: Peekable<I>) -> Self {
        Self { tokens }
    }

    pub(super) fn parse(mut self) -> Result<System> {
        let mut workflows = HashMap::new();
        while let Some((identifier, wf)) = self.workflow()? {
            workflows.insert(identifier, wf);
        }

        let mut parts = vec![];
        while let Some(part) = self.part()? {
            parts.push(part);
        }

        Ok(System { workflows, parts })
    }

    fn part(&mut self) -> Result<Option<Part>> {
        match self.tokens.peek() {
            Some(Token::OpenBrace) => (),
            _ => return Ok(None),
        };

        self.tokens.next();

        self.expect(Token::Coolness)?;
        self.expect(Token::Equals)?;
        let coolness = self.number()?;
        self.expect(Token::Comma)?;

        self.expect(Token::Musical)?;
        self.expect(Token::Equals)?;
        let musicality = self.number()?;
        self.expect(Token::Comma)?;

        self.expect(Token::Aerodynamic)?;
        self.expect(Token::Equals)?;
        let aerodynamic = self.number()?;
        self.expect(Token::Comma)?;

        self.expect(Token::Shiny)?;
        self.expect(Token::Equals)?;
        let shininess = self.number()?;

        self.expect(Token::CloseBrace)?;

        Ok(Some(Part {
            coolness,
            musicality,
            aerodynamic,
            shininess,
        }))
    }

    fn workflow(&mut self) -> Result<Option<(String, Workflow)>> {
        let identifier = match self.tokens.peek() {
            Some(Token::Identifier(identifier)) => identifier.clone(),
            _ => return Ok(None),
        };

        self.tokens.next();
        self.expect(Token::OpenBrace)?;

        let mut rules = vec![];
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Coolness | Token::Musical | Token::Aerodynamic | Token::Shiny => {
                    rules.push(self.rule()?);
                }
                Token::Identifier(_) | Token::Accept | Token::Reject => {
                    let workflow = Workflow {
                        rules,
                        otherwise: self.review_result()?,
                    };

                    self.expect(Token::CloseBrace)?;
                    return Ok(Some((identifier, workflow)));
                }
                _ => return Err(ParserError::ExpectedToken(Token::CloseBrace)),
            }

            self.expect(Token::Comma)?;
        }

        todo!()
    }

    fn rule(&mut self) -> Result<Rule> {
        let category = match self.tokens.next().ok_or(ParserError::UnexpectedEnd)? {
            Token::Coolness => Category::Coolness,
            Token::Musical => Category::Musicality,
            Token::Aerodynamic => Category::Aerodynamic,
            Token::Shiny => Category::Shininess,
            _ => return Err(ParserError::ExpectedCategory),
        };

        let result = match self.tokens.next().ok_or(ParserError::UnexpectedEnd)? {
            Token::OpenAngle => self.rule_less_than(category)?,
            Token::CloseAngle => self.rule_greater_than(category)?,
            _ => return Err(ParserError::ExpectedRule),
        };

        Ok(result)
    }

    fn rule_less_than(&mut self, category: Category) -> Result<Rule> {
        let value = self.number()?;
        self.expect(Token::Colon)?;
        let then = self.review_result()?;
        Ok(Rule::LessThan {
            category,
            value,
            then,
        })
    }

    fn rule_greater_than(&mut self, category: Category) -> Result<Rule> {
        let value = self.number()?;
        self.expect(Token::Colon)?;
        let then = self.review_result()?;
        Ok(Rule::GreaterThan {
            category,
            value,
            then,
        })
    }

    fn review_result(&mut self) -> Result<ReviewResult> {
        match self.tokens.next() {
            Some(Token::Accept) => Ok(ReviewResult::Accept),
            Some(Token::Reject) => Ok(ReviewResult::Reject),
            Some(Token::Identifier(identifier)) => Ok(ReviewResult::Pass(identifier)),
            _ => Err(ParserError::ExpectedToken(Token::Accept)),
        }
    }

    fn number(&mut self) -> Result<u64> {
        match self.tokens.next() {
            Some(Token::Number(n)) => Ok(n),
            _ => Err(ParserError::ExpectedToken(Token::Number(0))),
        }
    }

    fn expect(&mut self, token: Token) -> Result<Token> {
        match self.tokens.next() {
            Some(t) if t == token => Ok(t),
            _ => Err(ParserError::ExpectedToken(token)),
        }
    }
}
