use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Unexpected token here")]
#[diagnostic(code("Unexpected token found while lexing"), help("Try removing this"))]
pub struct UnexpectedTokenError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Mismatched parenthesis here")]
#[diagnostic(
    code("Mismatched parenthesis found during evaluation"),
    help("Try removing this")
)]
pub struct BadParenthesesError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Invalid operator here")]
#[diagnostic(
    code("Invalid operators found during parsing"),
    help("Try removing this")
)]
pub struct InvalidOperatorError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}
