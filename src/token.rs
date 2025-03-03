use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Token {
    EOF,
    Error(TokenError),
    Identifier(String),
    Keyword(Keyword),
    MultilineComment(String),
    Comment(String),

    //
    // Literals
    //
    BoolLiteral(bool),
    CharLiteral(String),
    NumberLiteral(Number),
    StringLiteral(String),
    TemplateLiteral(String),

    //
    // Delimiters + Operators
    //
    Amp,              // &
    Asterisk,         // *
    At,               // @
    BSlash,           // \
    Backtick,         // `
    Bang,             // !
    Caret,            // ^
    Colon,            // :
    Comma,            // ,
    DblQuote,         // "
    DivideEqual,      // /=
    Dot,              // .
    Equal,            // =
    EqualTo,          // ==
    FSlash,           // /
    GreaterThan,      // >
    GreaterThanEqual, // >=
    LBrace,           // {
    LBracket,         // [
    LParen,           // (
    LessThan,         // <
    LessThanEqual,    // <=
    LogicalAnd,       // &&
    LogicalOr,        // ||
    Minus,            // -
    MinusEqual,       // -=
    MultiplyEqual,    // *=
    NotEqualTo,       // !=
    Percent,          // %
    Pipe,             // |
    Plus,             // +
    PlusEqual,        // +=
    Question,         // ?
    Quote,            // '
    RBrace,           // }
    RBracket,         // ]
    RParen,           // )
    Semi,             // ;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TokenError {
    Illegal(char),
    UnterminatedCharLiteral,
    UnterminatedStringLiteral,
    MalformedHexadecimal,
    MalformedDecimal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Keyword {
    AS,
    ASYNC,
    AWAIT,
    BREAK,
    CONST,
    CONTINUE,
    ELSE,
    FOR,
    FUNC,
    IF,
    IMPL,
    LET,
    MATCH,
    PUB,
    RETURN,
    SELF,
    TRAIT,
    TYPE,
    USE,
    VOID,
    WHERE,
    WHILE,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumberKind {
    Integer,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
    pub kind: NumberKind,
    pub value: String,
    pub postfix: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenFrame {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

impl TokenFrame {
    pub fn empty() -> Self {
        Self {
            start: 0,
            end: 0,
            token: Token::EOF,
        }
    }
}
