use std::error::Error;

use crate::Span;

#[derive(Debug)]
pub struct ParseError<'a> {
    input: Span<'a>,
    message: Option<String>,
}

enum ParseErrorKind {
    UnexpectedToken,
    BadParentheses,
    IncompleteFP,
}

impl<'a> ParseError<'a> {
    pub fn new(input: Span<'a>, message: String) -> Self {
        Self {
            input,
            message: Some(message),
        }
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self {
            input,
            message: Some(format!("Parse Error: {:?}", kind)),
        }
    }

    fn append(input: Span<'a>, kind: nom::error::ErrorKind, other: Self) -> Self {
        Self {
            input,
            message: Some(format!("Parse Error: {:?} + {:?}", kind, other.message)),
        }
    }
}

impl<'a, E: Error> nom::error::FromExternalError<Span<'a>, E> for ParseError<'a> {
    fn from_external_error(input: Span<'a>, kind: nom::error::ErrorKind, e: E) -> Self {
        Self {
            input,
            message: Some(format!("External Error: {:?}", kind.description())),
        }
    }
}

// #[derive(Error, Debug, Diagnostic)]
// #[error("Unexpected token here")]
// #[diagnostic(code("Unexpected token found while lexing"), help("Try removing this"))]
// pub struct UnexpectedTokenError {
//     #[source_code]
//     pub src: NamedSource<String>,
//     #[label("here")]
//     pub err_span: SourceSpan,
// }

// #[derive(Error, Debug, Diagnostic)]
// #[error("Mismatched parenthesis here")]
// #[diagnostic(
//     code("Mismatched parenthesis found during evaluation"),
//     help("Try removing this")
// )]
// pub struct BadParenthesesError {
//     #[source_code]
//     pub src: NamedSource<String>,
//     #[label("here")]
//     pub err_span: SourceSpan,
// }

// #[derive(Error, Debug, Diagnostic)]
// #[error("Incomplete instance of a floating point number")]
// #[diagnostic(
//     code("Lone period found without both an integer and a fractional part"),
//     help("Try completing this")
// )]
// pub struct IncompleteFPError {
//     #[source_code]
//     pub src: NamedSource<String>,
//     #[label("here")]
//     pub err_span: SourceSpan,
// }
