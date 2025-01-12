use miette::{NamedSource, Result, SourceOffset, SourceSpan};

use crate::{error::UnexpectedTokenError, token_arr_to_number};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Numeric(Number),
    Whitespace,
    Op(Operator),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftParen => f.write_str("("),
            Token::RightParen => f.write_str(")"),
            Token::Numeric(number) => f.write_str(&number.to_string()),
            Token::Whitespace => f.write_str(" "),
            Token::Op(operator) => f.write_str(&operator.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub f64);

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Number(fp) = self;
        f.write_str(&fp.to_string())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    // /
    Slash,
}

impl Operator {
    pub fn has_greater_precedence_than(&self, other_op: Self) -> bool {
        match self {
            Operator::Plus | Operator::Minus => false,
            Operator::Asterisk => !matches!(other_op, Operator::Slash | Operator::Asterisk),
            Operator::Slash => !matches!(other_op, Operator::Slash),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => f.write_str("+"),
            Operator::Minus => f.write_str("-"),
            Operator::Asterisk => f.write_str("*"),
            Operator::Slash => f.write_str("/"),
        }
    }
}

pub struct Lexer<'a> {
    src: &'a str,
    current_token: Option<char>,
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        Self {
            src: expr_as_string,
            current_token: None,
            rest: expr_as_string,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        self.advance();
        while self.current_token.is_some() {
            match self.tokenize_character(self.current_token.unwrap()) {
                Some(t) => {
                    // Ignore whitespace as a token.
                    if let Token::Whitespace = t {
                        // Need this so does not end in an infinite loop.
                        self.advance();
                        continue;
                    }
                    tokens.push(t)
                }
                _ => {
                    let column = self.src.find(self.current_token.unwrap()).unwrap() + 1;
                    Err(UnexpectedTokenError {
                        src: NamedSource::new("mathexpr", self.src.to_owned()),
                        err_span: {
                            let start = SourceOffset::from_location(self.src, 1, column);
                            SourceSpan::new(start, 1)
                        },
                    })?;
                }
            };

            self.advance();
        }

        // Group adjacent `Numbers` into a single one.
        // Similar to the evaluation of RPN, this adds numbers to a vector until it reaches an operator or another token.
        // Then, it tries to unify each digit into one number and pushes it to a final token vector along with the operator.
        let mut grouped_tokens: Vec<Token> = vec![];
        let mut tokens_to_group: Vec<&Token> = vec![];
        let mut tokens_to_eat = tokens.len() - 1;

        for token in &tokens {
            if tokens_to_eat == 0 {
                tokens_to_group.push(token);
                let grouped_number = token_arr_to_number(&tokens_to_group);
                grouped_tokens.push(Token::Numeric(Number(grouped_number)));
                break;
            };
            match token {
                Token::Numeric(_) => tokens_to_group.push(token),
                _ => {
                    let grouped_number = token_arr_to_number(&tokens_to_group);
                    grouped_tokens.push(Token::Numeric(Number(grouped_number)));
                    grouped_tokens.push(token.clone());
                    tokens_to_group.clear();
                }
            }
            tokens_to_eat -= 1;
        }

        Ok(grouped_tokens)
    }

    fn tokenize_character(&mut self, character: char) -> Option<Token> {
        match character {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '0'..='9' => Some(Token::Numeric(Number(
                character.to_digit(10).unwrap() as f64
            ))),
            '+' => Some(Token::Op(Operator::Plus)),
            '-' => Some(Token::Op(Operator::Minus)),
            '*' => Some(Token::Op(Operator::Asterisk)),
            '/' => Some(Token::Op(Operator::Slash)),
            ' ' => Some(Token::Whitespace),
            _ => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if !self.rest.is_empty() {
            self.current_token = Some(self.rest.as_bytes()[0] as char);
            self.rest = &self.rest[1..];
            self.current_token
        } else {
            self.current_token = None;
            None
        }
    }

    pub fn src(&self) -> String {
        self.src.to_owned()
    }
}
