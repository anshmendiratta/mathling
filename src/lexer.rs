#[allow(unused_imports)]
use std::{collections::HashMap, fmt::Write, ops::Range};

use nom::{branch::alt, character::streaming::multispace0};

use crate::{
    util::{find_equal, find_fp},
    IResult, Span,
};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Token {
//     pub kind: TokenKind,
//     pub column: usize,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum TokenKind {
//     // For values.
//     LeftParen,
//     RightParen,
//     Numeric(Number),
//     Whitespace,
//     Period,
//     Op(BinOp),
//     // For parts of variables. Do not use directly in parser.
//     Alphabetical(char),
//     // For identifier.
//     Identifier(String),
//     Equal,
//     Semicolon, // To separate statements.
// }

// impl std::fmt::Display for TokenKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TokenKind::LeftParen => f.write_str("("),
//             TokenKind::RightParen => f.write_str(")"),
//             TokenKind::Numeric(number) => f.write_str(&number.to_string()),
//             TokenKind::Whitespace => f.write_str(" "),
//             TokenKind::Op(operator) => f.write_str(&operator.to_string()),
//             TokenKind::Period => f.write_str("."),
//             TokenKind::Alphabetical(c) => f.write_char(*c),
//             TokenKind::Equal => f.write_str("="),
//             TokenKind::Semicolon => f.write_str(";"),
//             TokenKind::Identifier(id) => f.write_str(id),
//         }
//     }
// }

// impl Token {
//     pub fn kind(&self) -> TokenKind {
//         let Token { kind: k, .. } = self;
//         k.clone()
//     }

//     pub fn col(&self) -> usize {
//         let Token { column: c, .. } = self;
//         c.clone()
//     }
// }

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Value(fp) => f.write_str(&fp.to_string()),
            Expr::Id(i) => f.write_str(i),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
}

impl BinOp {
    pub fn has_greater_precedence_than(&self, other_op: &Self) -> bool {
        match self {
            BinOp::Plus | BinOp::Minus => false,
            BinOp::Times => !matches!(other_op, BinOp::Divide | BinOp::Times),
            BinOp::Divide => !matches!(other_op, BinOp::Divide),
            BinOp::Equal => false,
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Plus => f.write_str("+"),
            BinOp::Minus => f.write_str("-"),
            BinOp::Times => f.write_str("*"),
            BinOp::Divide => f.write_str("/"),
            BinOp::Equal => f.write_str("="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(f32),
    Id(String),
    BinOp(BinOp, Box<(Expr, Expr)>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Expr),
    Print(Expr),
}

pub struct Lexer<'a> {
    src: Span<'a>,
    current_token: Option<char>,
    current_col: usize,
    rest: Span<'a>,
    // Keep track of assignments. Ignore order.
    // pub assignments: HashMap<String, f64>,
}

impl<'a> Lexer<'a> {
    pub fn new(expr_as_string: &'a str) -> Self {
        Self {
            src: Span::new(expr_as_string),
            current_token: None,
            current_col: 0,
            rest: Span::new(expr_as_string),
        }
    }

    pub fn lex(&mut self) -> IResult<Vec<Statement>> {
        let (_, single_tokens) = self.parse_all()?;
        // let (_, grouped_number_tokens) = self.do_token_grouping_pass(single_tokens)?;
        // let tokens_with_fp = self.do_fp_lexing_pass(grouped_number_tokens)?;
        // let tokens_with_assignment = self.assemble_statements_pass(tokens_with_fp)?;

        // Ok(tokens_with_fp)
        Ok((Span::new(""), vec![]))
    }

    fn parse_all(&mut self) -> IResult<Vec<Statement>> {
        let mut statements: Vec<Statement> = vec![];
        let mut src_rest = self.src;
        let mut column_idx = 0;

        while !src_rest.is_empty() {
            let (src, spaces) = multispace0(src_rest)?;
            let d = alt((find_fp, find_op, find_equal)).parse(src);
            column_idx += spaces.len() + /* token_str */ 1;
            // let token = match self.tokenize_character(number_tokens) {
            //     Some(tkn) => single_tokens.push(Token {
            //         kind: tkn,
            //         column: column_idx,
            //     }),
            //     None => panic!(
            //         "{:?}",
            //         UnexpectedTokenError {
            //             src: NamedSource::new("mathexpr", self.src.to_owned()),
            //             err_span: {
            //                 let start = SourceOffset::from_location(self.src, 1, self.current_col);
            //                 SourceSpan::new(start, 1)
            //             },
            //         }
            //     ),
            // };
            // dbg!(&single_tokens);
            src_rest = src;
        }

        Ok((Span::new(""), statements))
    }

    /// Second pass:
    /// Group adjacent `Numbers` into a single one. Similar to the evaluation of RPN, this adds numbers to a vector until it reaches an operator or another token. Then, it tries to unify each digit into one number and pushes it to a final token vector along with the operator.
    // fn do_token_grouping_pass(&mut self, single_tokens: Vec<Token>) -> Result<Vec<Token>> {
    //     let mut grouped_tokens: Vec<Token> = vec![];
    //     let mut tokens_to_group: Vec<Token> = vec![];
    //     let mut follows_period: bool = false;
    //     for (i, token) in single_tokens.iter().enumerate() {
    //         let tokens_to_eat = single_tokens.len() - i - 1;
    //         if tokens_to_eat == 0 {
    //             // If no more tokens to add after this:
    //             // 1. Add the final token to the stack if it is a:
    //             // - Number => Send array to `token_arr_to_number` and append to `grouped_tokens`.
    //             // - Other => Append to `grouped_tokens`.
    //             if let TokenKind::Numeric(_) = token.kind {
    //                 tokens_to_group.push(token.clone());
    //             }
    //             if !tokens_to_group.is_empty() {
    //                 let (mut grouped_number, col_to_use) = token_arr_to_number(&tokens_to_group);
    //                 if follows_period {
    //                     grouped_number /= 10_f32.powf(tokens_to_group.len() as f32);
    //                 }
    //                 grouped_tokens.push(Token {
    //                     kind: TokenKind::Numeric(Number(grouped_number)),
    //                     column: col_to_use,
    //                 });
    //             }
    //             match token.kind {
    //                 TokenKind::Numeric(_) => (),
    //                 _ => grouped_tokens.push(token.clone()),
    //             }
    //             break;
    //         };
    //         match token {
    //             Token {
    //                 kind: TokenKind::Numeric(_),
    //                 ..
    //             } => tokens_to_group.push(token.clone()),
    //             Token {
    //                 kind: TokenKind::Period,
    //                 ..
    //             } => {
    //                 follows_period = true;
    //                 let (mut grouped_number, col_to_use) = token_arr_to_number(&tokens_to_group);
    //                 grouped_tokens.push(Token {
    //                     kind: TokenKind::Numeric(Number(grouped_number)),
    //                     column: col_to_use,
    //                 });
    //                 grouped_tokens.push(token.clone());
    //                 tokens_to_group.clear();
    //             }
    //             _ => {
    //                 if !tokens_to_group.is_empty() {
    //                     let (mut grouped_number, col_to_use) =
    //                         token_arr_to_number(&tokens_to_group);
    //                     if follows_period {
    //                         grouped_number /= 10_f32.powf(tokens_to_group.len() as f32);
    //                         follows_period = false;
    //                     }
    //                     grouped_tokens.push(Token {
    //                         kind: TokenKind::Numeric(Number(grouped_number)),
    //                         column: col_to_use,
    //                     });
    //                 }
    //                 grouped_tokens.push(token.clone());
    //                 tokens_to_group.clear();
    //             }
    //         }
    //     }

    //     Ok(grouped_tokens)
    // }

    /// Second pass:
    /// Convert `num, period, num` into `num.num` (lex floating point).
    // fn do_fp_lexing_pass(&mut self, grouped_tokens: Vec<Token>) -> Result<Vec<Token>> {
    //     let mut tokens = vec![];
    //     let mut fp_stack: Vec<Token> = vec![];
    //     let mut follows_period = false;
    //     for token in grouped_tokens {
    //         match token {
    //             Token {
    //                 kind: TokenKind::Numeric(_),
    //                 ..
    //             } => {
    //                 fp_stack.push(token);

    //                 if follows_period {
    //                     follows_period = false;
    //                     let fract = match fp_stack.pop() {
    //                         Some(Token {
    //                             kind: TokenKind::Numeric(Number(fract)),
    //                             ..
    //                         }) => fract,
    //                         _ => panic!(" "),
    //                     };
    //                     let _ = fp_stack.pop();
    //                     let mut col_to_use;
    //                     let int = match fp_stack.pop() {
    //                         Some(Token {
    //                             kind: TokenKind::Numeric(Number(int)),
    //                             column: col,
    //                         }) => {
    //                             col_to_use = col;
    //                             int
    //                         }
    //                         _ => panic!(" "),
    //                     };
    //                     let fp = Token {
    //                         kind: TokenKind::Numeric(Number(int + fract)),
    //                         column: col_to_use,
    //                     };

    //                     tokens.append(&mut fp_stack);
    //                     tokens.push(fp);
    //                 }
    //             }
    //             Token {
    //                 kind: TokenKind::Period,
    //                 ..
    //             } => {
    //                 follows_period = true;
    //                 fp_stack.push(token);
    //             }
    //             _ => fp_stack.push(token),
    //         }
    //     }

    //     if !fp_stack.is_empty() {
    //         tokens.append(&mut fp_stack);
    //     }

    //     Ok(tokens)
    // }

    // Third pass: separate statements with `;` and store any assignment that are made. Finally, a stream of tokens is returned with the adjacent characters grouped into a single identifier.
    // TODO(Ansh): Make it less ugly.
    // fn assemble_statements_pass(&mut self, fp_tokens: Vec<Token>) -> Result<Vec<Token>> {
    //     // Find the indices of all the semicolons in the `fp_tokens`.
    //     let mut semicolon_idxs: Vec<isize> = vec![-1];
    //     semicolon_idxs.append(
    //         &mut fp_tokens
    //             .iter()
    //             .enumerate()
    //             .filter(|(_, t)| t.kind == TokenKind::Semicolon)
    //             .map(|(idx, _)| idx as isize)
    //             .collect(),
    //     );

    //     // Find and verify all the ranges of indices in `fp_tokens` that are assignments. All ranges are inclusive.
    //     let mut assignment_ranges: Vec<Range<isize>> = Vec::new();
    //     for window in semicolon_idxs.windows(2) {
    //         // This is ugly. I'd rather just do `for [a, b] in semicolon_idxs...` instead of pattern matching with the if-let and introducing another indentation level.
    //         if let [a, b] = window {
    //             let tokens_in_range: Vec<Token> = ((*a + 1)..*b)
    //                 .into_iter()
    //                 .map(|x| fp_tokens[x as usize].clone())
    //                 .collect();
    //             let has_equal_token = tokens_in_range
    //                 .iter()
    //                 .map(|t| t.kind.clone())
    //                 .collect::<Vec<TokenKind>>()
    //                 .contains(&TokenKind::Equal);

    //             if has_equal_token {
    //                 assignment_ranges.push((*a + 1)..(*b));
    //             }
    //         }
    //     }

    //     for assignment_range in &assignment_ranges {
    //         let assignment_tokens: Vec<Token> = assignment_range
    //             .clone()
    //             .map(|i| fp_tokens[i as usize].clone())
    //             .collect();
    //         let equal_idx = assignment_tokens
    //             .iter()
    //             .position(|e| e.kind == TokenKind::Equal)
    //             .expect("LEXER: Incorrectly determined an assignment range.");
    //         let identifier_tokens = &assignment_tokens[0..equal_idx];
    //         let value_tokens = &assignment_tokens[(equal_idx + 1 as usize)..];
    //         // Check that all the tokens used for the identifier are alphabetical.
    //         assert!(
    //             identifier_tokens
    //                 .iter()
    //                 .filter(|t| {
    //                     if let TokenKind::Alphabetical(_) = t.kind {
    //                         return true;
    //                     }
    //                     return false;
    //                 })
    //                 .collect::<Vec<_>>()
    //                 .len()
    //                 == identifier_tokens.len()
    //         );
    //         let identifier = identifier_tokens
    //             .iter()
    //             .map(|t| {
    //                 if let TokenKind::Alphabetical(c) = t.kind {
    //                     c.to_string()
    //                 } else {
    //                     panic!("LEXER: Found non-character in identifier in assignment.");
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //             .join("");
    //         let value = Number(token_arr_to_number(value_tokens).0);

    //         // Add assignment
    //         dbg!(&identifier, &value);
    //         self.assignments.insert(identifier, value);
    //         // Delete assignment tokens from the return so the parser is unchanged.
    //         // tokens.retain(|t| !assignment_tokens.contains(t) && t.kind != TokenKind::Semicolon)
    //     }

    //     // Tokens to return.
    //     let mut tokens = vec![];
    //     // Look at just the expression to evaluate.
    //     let mut last_semicolon_idx = 0;
    //     if let Some(last_assignment_range) = assignment_ranges.last() {
    //         last_semicolon_idx = last_assignment_range.end;
    //     }
    //     // Iterate over the last statement.
    //     let mut alphabetical_queue: Vec<Token> = vec![];
    //     for token in &fp_tokens[(last_semicolon_idx as usize + 1)..] {
    //         match token.kind {
    //             TokenKind::Alphabetical(_) => {
    //                 alphabetical_queue.push(token.clone());
    //             }
    //             _ => {
    //                 if !alphabetical_queue.is_empty() {
    //                     let identifier_token = alphabetical_arr_to_identifier(&alphabetical_queue);
    //                     tokens.push(identifier_token);
    //                     alphabetical_queue.clear();
    //                 }

    //                 // Push whatever non-identifier is the current iteration.
    //                 tokens.push(token.clone());
    //             }
    //         }
    //     }

    //     // Clean up the queue in case the statement ends with an identifier.
    //     if !alphabetical_queue.is_empty() {
    //         let identifier_token = alphabetical_arr_to_identifier(&alphabetical_queue);
    //         tokens.push(identifier_token);
    //         alphabetical_queue.clear();
    //     }

    //     Ok(tokens)
    // }

    // fn tokenize_character(&mut self, character: char) -> Option<TokenKind> {
    //     match character {
    //         '(' => Some(TokenKind::LeftParen),
    //         ')' => Some(TokenKind::RightParen),
    //         '0'..='9' => Some(TokenKind::Numeric(Number(
    //             character.to_digit(10).unwrap() as f32
    //         ))),
    //         '+' => Some(TokenKind::Op(BinOp::Plus)),
    //         '-' => Some(TokenKind::Op(BinOp::Minus)),
    //         '*' => Some(TokenKind::Op(BinOp::Times)),
    //         '/' => Some(TokenKind::Op(BinOp::Slash)),
    //         ' ' => Some(TokenKind::Whitespace),
    //         '.' => Some(TokenKind::Period),
    //         'a'..='z' | 'A'..='Z' => Some(TokenKind::Alphabetical(character)),
    //         '=' => Some(TokenKind::Equal),
    //         ';' => Some(TokenKind::Semicolon),
    //         _ => None,
    //     }
    // }

    // fn advance(&mut self) -> Option<char> {
    //     if !self.rest.is_empty() {
    //         self.current_token = Some(self.rest.as_bytes()[0] as char);
    //         self.rest = &self.rest[1..];
    //         self.current_col += 1;
    //         self.current_token
    //     } else {
    //         self.current_token = None;
    //         None
    //     }
    // }

    pub fn src(&self) -> Span {
        self.src
    }

    pub fn current_idx(&self) -> usize {
        self.current_col
    }

    // pub fn assignments(&self) -> HashMap<String, Number> {
    //     self.assignments.clone()
    // }
}
