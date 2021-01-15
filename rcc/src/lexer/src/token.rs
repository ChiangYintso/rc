use strenum::EnumFromStr;

#[derive(Debug, PartialEq, EnumFromStr)]
pub enum Token {
    /// strict keywords
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mut,
    Ref,
    Return,

    #[value("self")]
    SelfValue,

    #[value("Self")]
    SelfType,

    Static,
    Struct,
    True,
    While,

    /// reserved keywords
    Crate,
    Mod,
    Move,
    Super,
    Trait,
    Type,
    Unsafe,
    Use,
    Where,
    Async,
    Await,
    Dyn,
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,
    Try,
    Union,

    #[disabled]
    Identifier(String),

    #[disabled]
    Literals {
        literal_kind: LiteralKind,
    },

    /// symbols
    #[value("+")]
    Plus,

    #[value("-")]
    Minus,

    #[value("*")]
    Star,

    #[value("/")]
    Slash,

    #[value("%")]
    Percent,

    #[value("^")]
    Caret,

    #[value("!")]
    Not,

    #[value("&")]
    And,

    #[value("|")]
    Or,

    #[value("&&")]
    AndAnd,

    #[value("||")]
    OrOr,

    #[value("<<")]
    Shl,

    #[value(">>")]
    Shr,

    #[value("+=")]
    PlusEq,

    #[value("-=")]
    MinusEq,

    #[value("*=")]
    StarEq,

    #[value("/=")]
    SlashEq,

    #[value("%=")]
    PercentEq,

    #[value("^=")]
    CaretEq,

    #[value("<<=")]
    ShlEq,

    #[value(">>=")]
    ShrEq,

    #[value("=")]
    Eq,

    #[value("==")]
    EqEq,

    #[value("!=")]
    Ne,

    #[value(">")]
    Gt,

    #[value("<")]
    Lt,

    #[value(">=")]
    Ge,

    #[value("<=")]
    Le,

    #[value("@")]
    At,

    #[value("_")]
    Underscore,

    #[value(".")]
    Dot,

    #[value("..")]
    DotDot,

    #[value("...")]
    DotDotDot,

    #[value("..=")]
    DotDotEq,

    #[value(",")]
    Comma,

    #[value(";")]
    Semi,

    #[value(":")]
    Colon,

    #[value("::")]
    PathSep,

    #[value("->")]
    RArrow,

    #[value("=>")]
    FatArrow,

    #[value("#")]
    Pound,

    #[value("$")]
    Dollar,

    #[value("?")]
    Question,

    /// delimiters
    #[value("{")]
    LeftCurlyBraces,

    #[value("}")]
    RightCurlyBraces,

    #[value("[")]
    LeftSquareBrackets,

    #[value("]")]
    RightSquareBrackets,

    #[value("(")]
    LeftParentheses,

    #[value(")")]
    RightParentheses,

    #[disabled]
    WhiteSpace,

    #[disabled]
    Comment,

    #[disabled]
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Integer,
    Float,
    String,
    Char,
}
