use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    pub line: u32,
    pub col: u32,
}

impl Loc {
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),

    // Keywords
    Struct,
    Enum,
    Trait,
    Impl,
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Yield,
    Break,
    True,
    False,
    Null,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    ColonColon,
    Semicolon,

    // Arithmetic Operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,

    // Boolean/Logic Operators
    LogNot,
    BitNot,
    LogAnd,
    BitAnd,
    LogOr,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,

    // Assignment Operators
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    ModEqual,
    BitAndEqual,
    BitOrEqual,
    BitXorEqual,
    LeftShiftEqual,
    RightShiftEqual,

    // Comparison Operators
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    EOF,
}

impl TokenValue {
    pub fn precedence(&self) -> Option<u8> {
        match self {
            Self::Comma => Some(0),
            Self::Equal
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::ModEqual
            | Self::RightShiftEqual
            | Self::LeftShiftEqual
            | Self::BitAndEqual
            | Self::BitOrEqual
            | Self::BitXorEqual => Some(1),
            Self::LogOr => Some(3),
            Self::LogAnd => Some(4),
            Self::BitOr => Some(5),
            Self::BitXor => Some(6),
            Self::BitAnd => Some(7),
            Self::EqualEqual | Self::NotEqual => Some(8),
            Self::Less | Self::LessEqual | Self::Greater | Self::GreaterEqual => Some(9),
            Self::LeftShift | Self::RightShift => Some(10),
            Self::Plus | Self::Minus => Some(11),
            Self::Star | Self::Slash | Self::Mod => Some(12),
            Self::LogNot | Self::BitNot => Some(13),
            _ => None,
        }
    }

    pub fn assoc(&self) -> Option<Assoc> {
        match self {
            Self::Equal
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::ModEqual
            | Self::RightShiftEqual
            | Self::LeftShiftEqual
            | Self::BitAndEqual
            | Self::BitOrEqual
            | Self::BitXorEqual
            | Self::LogNot
            | Self::BitNot => Some(Assoc::Right),
            Self::Comma
            | Self::LogOr
            | Self::LogAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitAnd
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::LeftShift
            | Self::RightShift
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Mod => Some(Assoc::Left),
            _ => None,
        }
    }

    pub fn is_func(&self, known_funcs: &Vec<String>) -> bool {
        match self {
            Self::Identifier(str) if known_funcs.contains(&str) => true,
            _ => false,
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            Self::LeftParen
            | Self::Equal
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::ModEqual
            | Self::RightShiftEqual
            | Self::LeftShiftEqual
            | Self::BitAndEqual
            | Self::BitOrEqual
            | Self::BitXorEqual
            | Self::LogNot
            | Self::BitNot
            // | Self::Comma
            | Self::LogOr
            | Self::LogAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitAnd
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::LeftShift
            | Self::RightShift
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Mod => true,
            _ => false,
        }
    }

    pub fn is_binding(&self, known_bindings: &Vec<String>) -> bool {
        match self {
            Self::Identifier(str) if known_bindings.contains(&str) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    start_loc: Loc,
    end_loc: Loc,
    pub value: TokenValue,
}

impl Token {
    pub fn new(start_loc: Loc, end_loc: Loc, value: TokenValue) -> Self {
        Self {
            start_loc,
            end_loc,
            value,
        }
    }
}

lazy_static! {
    pub static ref KEYWORD_TABLE: HashMap<String, TokenValue> = HashMap::from([
        ("struct".to_string(), TokenValue::Struct),
        ("enum".to_string(), TokenValue::Enum),
        ("trait".to_string(), TokenValue::Trait),
        ("impl".to_string(), TokenValue::Impl),
        ("fn".to_string(), TokenValue::Fn),
        ("let".to_string(), TokenValue::Let),
        ("mut".to_string(), TokenValue::Mut),
        ("if".to_string(), TokenValue::If),
        ("else".to_string(), TokenValue::Else),
        ("while".to_string(), TokenValue::While),
        ("for".to_string(), TokenValue::For),
        ("in".to_string(), TokenValue::In),
        ("return".to_string(), TokenValue::Return),
        ("yield".to_string(), TokenValue::Yield),
        ("break".to_string(), TokenValue::Break),
        ("true".to_string(), TokenValue::True),
        ("false".to_string(), TokenValue::False),
        ("null".to_string(), TokenValue::Null),
    ]);
}
