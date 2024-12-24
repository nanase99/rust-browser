use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    /// https://www.w3.org/TR/css-syntax-3/#typeof-hash-token
    HashToken(String),
    /// https://www.w3.org/TR/css-syntax-3/#typedef-delim-token
    Delim(char),
    /// https://www.w3.org/TR/css-syntax-3/#typedef-number-token
    Number(f64),
    /// https://www.w3.org/TR/css-syntax-3/#typedef-colon-token
    Colon,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-semicolon-token
    Semicolon,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-open-paren
    OpenParenthesis,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-close-paren
    CloseParenthesis,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-open-curly
    OpenCurly,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-close-curly
    CloseCurly,
    /// https://www.w3.org/TR/css-syntax-3/#typedef-ident-token
    Ident(String),
    /// https://www.w3.org/TR/css-syntax-3/#typedef-string-token
    StringToken(String),
    /// https://www.w3.org/TR/css-syntax-3/#typedef-at-keyword-token
    AtKeyword(String),
}
