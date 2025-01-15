use miette::{NamedSource, Result, SourceOffset, SourceSpan};

use crate::{error::UnexpectedTokenError, token_arr_to_number};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Numeric(Number),
    Whitespace,
    Period,
    Op(Operator),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LeftParen => f.write_str("("),
            TokenKind::RightParen => f.write_str(")"),
            TokenKind::Numeric(number) => f.write_str(&number.to_string()),
            TokenKind::Whitespace => f.write_str(" "),
            TokenKind::Op(operator) => f.write_str(&operator.to_string()),
            TokenKind::Period => f.write_str("."),
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
    Slash,
}

impl Operator {
    pub fn has_greater_precedence_than(&self, other_op: &Self) -> bool {
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
    current_col: usize,
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        Self {
            src: expr_as_string,
            current_token: None,
            current_col: 0,
            rest: expr_as_string,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut single_tokens: Vec<Token> = vec![];

        self.advance();
        self.current_col = 0;
        while self.current_token.is_some() {
            match self.tokenize_character(self.current_token.unwrap()) {
                Some(tk) => {
                    // Don't tokenize whitespaces.
                    if let TokenKind::Whitespace = tk {
                        self.advance();
                        // self.current_col -= 1;
                        continue;
                    }
                    single_tokens.push(Token {
                        kind: tk,
                        col: self.current_col,
                    });
                }
                _ => {
                    Err(UnexpectedTokenError {
                        src: NamedSource::new("mathexpr", self.src.to_owned()),
                        err_span: {
                            let start = SourceOffset::from_location(self.src, 1, self.current_col);
                            SourceSpan::new(start, 1)
                        },
                    })?;
                }
            };
            self.advance();
        }

        // First pass:
        // Group adjacent `Numbers` into a single one.
        // Similar to the evaluation of RPN, this adds numbers to a vector until it reaches an operator or another token.
        // Then, it tries to unify each digit into one number and pushes it to a final token vector along with the operator.
        let mut grouped_tokens: Vec<Token> = vec![];
        let mut tokens_to_group: Vec<Token> = vec![];
        for (i, token) in single_tokens.iter().enumerate() {
            let tokens_to_eat = single_tokens.len() - i - 1;
            if tokens_to_eat == 0 {
                // If no more tokens to add after this:
                // 1. Add the final token to the stack if it is a:
                // - Number => Send array to `token_arr_to_number` and append to `grouped_tokens`.
                // - Other => Append to `grouped_tokens`.
                match token.kind {
                    TokenKind::Numeric(_) => tokens_to_group.push(token.clone()),
                    _ => (),
                }
                if !tokens_to_group.is_empty() {
                    let (grouped_number, col_to_use) = token_arr_to_number(&tokens_to_group);
                    grouped_tokens.push(Token {
                        kind: TokenKind::Numeric(Number(grouped_number)),
                        col: col_to_use,
                    });
                }
                match token.kind {
                    TokenKind::Numeric(_) => (),
                    _ => grouped_tokens.push(token.clone()),
                }
                break;
            };
            match token {
                Token {
                    kind: TokenKind::Numeric(_),
                    ..
                } => tokens_to_group.push(token.clone()),
                _ => {
                    if !tokens_to_group.is_empty() {
                        let (grouped_number, col_to_use) = token_arr_to_number(&tokens_to_group);
                        grouped_tokens.push(Token {
                            kind: TokenKind::Numeric(Number(grouped_number)),
                            col: col_to_use,
                        });
                    }
                    grouped_tokens.push(token.clone());
                    tokens_to_group.clear();
                    continue;
                }
            }
        }

        // Second pass:
        // Convert `num, period, num` into `num.num` (lex floating point).
        // Each (usize, usize) in `period_indices` is a pair of a index in `grouped_tokens` and the `col` in `self.src`.
        // let period_indices: Vec<(usize, usize)> = grouped_tokens
        //     .iter()
        //     .enumerate()
        //     .filter_map(|p| {
        //         if p.1.kind == TokenKind::Period {
        //             Some((p.0, p.1.col))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        // let mut fp_idx_triplets = vec![];
        // let floating_points: Vec<Token> = period_indices
        //     .iter()
        //     .map(|(idx, col)| -> Result<Token> {
        //         if *idx < 1 as usize || *idx > grouped_tokens.len() - 2 {
        //             Err(IncompleteFPError {
        //                 src: NamedSource::new("mathexpr", self.src.to_owned()),
        //                 err_span: {
        //                     let start = SourceOffset::from_location(self.src, 1, *col + 1);
        //                     SourceSpan::new(start, 1)
        //                 },
        //             })?
        //         }

        //         let (int, fract) = (
        //             // Because of the above `if`, this will never fail.
        //             grouped_tokens[idx - 1].clone(),
        //             grouped_tokens[idx + 1].clone(),
        //         );
        //         fp_idx_triplets.push((idx - 1, idx, idx + 1));
        //         let number = match (int.kind, fract.kind) {
        //             (TokenKind::Numeric(Number(int)), TokenKind::Numeric(Number(fract))) => {
        //                 int + fract / (10_f64.powf(fract.to_string().len() as f64))
        //             }
        //             _ => Err(IncompleteFPError {
        //                 src: NamedSource::new("mathexpr", self.src.to_owned()),
        //                 err_span: {
        //                     let start = SourceOffset::from_location(self.src, 1, *col + 1);
        //                     SourceSpan::new(start, 1)
        //                 },
        //             })?,
        //         };
        //         Ok(Token {
        //             kind: TokenKind::Numeric(Number(number)),
        //             col: int.col,
        //         })
        //     })
        //     .collect::<Result<Vec<Token>>>()?;
        let tokens = {
            let mut tokens_bind = vec![];
            for window in grouped_tokens.windows(5) {
                match &window[..] {
                    [a, b, Token {
                        kind: TokenKind::Period,
                        ..
                    }] => {
                        tokens_bind.push(a.clone());
                    }
                    [a, Token {
                        kind: TokenKind::Period,
                        col,
                    }, c] => {
                        let (a, c) = match (a, c) {
                            (
                                Token {
                                    kind: TokenKind::Numeric(Number(n_1)),
                                    ..
                                },
                                Token {
                                    kind: TokenKind::Numeric(Number(n_2)),
                                    ..
                                },
                            ) => (n_1, n_2),
                            _ => panic!(" "),
                        };
                        let fp = a + c / (10_f64.powf(c.to_string().len() as f64));
                        let fp_tk = Token {
                            kind: TokenKind::Numeric(Number(fp)),
                            col: *col,
                        };
                        tokens_bind.push(fp_tk);
                    }
                    [Token {
                        kind: TokenKind::Period,
                        ..
                    }, b, c] => {
                        tokens_bind.push(c.clone());
                    }
                    [a, b, c] => {
                        tokens_bind.push(a.clone());
                        tokens_bind.push(b.clone());
                        tokens_bind.push(c.clone());
                    }
                    _ => (),
                }
            }

            tokens_bind
        };
        dbg!(tokens);
        // Ok(floating_points)
        Ok(grouped_tokens)
    }

    fn tokenize_character(&mut self, character: char) -> Option<TokenKind> {
        match character {
            '(' => Some(TokenKind::LeftParen),
            ')' => Some(TokenKind::RightParen),
            '0'..='9' => Some(TokenKind::Numeric(Number(
                character.to_digit(10).unwrap() as f64
            ))),
            '+' => Some(TokenKind::Op(Operator::Plus)),
            '-' => Some(TokenKind::Op(Operator::Minus)),
            '*' => Some(TokenKind::Op(Operator::Asterisk)),
            '/' => Some(TokenKind::Op(Operator::Slash)),
            ' ' => Some(TokenKind::Whitespace),
            '.' => Some(TokenKind::Period),
            _ => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if !self.rest.is_empty() {
            self.current_token = Some(self.rest.as_bytes()[0] as char);
            self.rest = &self.rest[1..];
            self.current_col += 1;
            self.current_token
        } else {
            self.current_token = None;
            None
        }
    }

    pub fn src(&self) -> String {
        self.src.to_owned()
    }

    pub fn current_idx(&self) -> usize {
        self.current_col
    }
}
