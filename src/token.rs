use core::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}
#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    Identifier,
    Intiger,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Comma,
    Semicolon,

    Eq,
    NotEq,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "Illegal"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Intiger => write!(f, "Intiger"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Gt => write!(f, ">"),
            TokenType::Lt => write!(f, "<"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Let => write!(f, "Let"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Return => write!(f, "Return"),
            TokenType::NotEq => write!(f, "NotEq"),
            TokenType::Eq => write!(f, "Eq"),
        }
    }
}

pub fn lookup_identifier(identifier: &String) -> TokenType {
    match identifier.as_str() {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Identifier,
    }
}
