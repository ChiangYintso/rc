use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use strenum::StrEnum;

use crate::lexer::token::Token;
use crate::lexer::token::Token::{Minus, Not, Star};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Path(PathExpr),
    Lit(LitExpr),
    Unary(UnAryExpr),
    Block(BlockExpr),
    Borrow(BorrowExpr),
    BinOp(BinOpExpr),
    Group(GroupExpr),
    Array(ArrayExpr),
    ArrayIndex(ArrayIndexExpr),
    Tuple(TupleExpr),
    TupleIndex(TupleIndexExpr),
    Struct(StructExpr),
    EnumVariant,
    Call,
    MethodCall,
    FieldAccess,
    Loop,
    Range,
    If,
    Match,
    Return,
    Nothing,
}

#[derive(Debug, PartialEq)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

#[derive(PartialEq, Debug)]
pub struct LitExpr {
    pub ret_type: String,
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct PathExpr {
    pub segments: Vec<String>,
}

impl PathExpr {
    pub fn new() -> Self {
        PathExpr { segments: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnAryExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(PartialEq)]
pub enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

impl Debug for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::Deref => '*',
            Self::Not => '!',
            Self::Neg => '-',
        })
    }
}

impl UnOp {
    pub fn from_token(tk: &Token) -> Option<Self> {
        match tk {
            Minus => Some(Self::Neg),
            Star => Some(Self::Deref),
            Not => Some(Self::Not),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct BorrowExpr {
    borrow_cnt: u32,
    is_mut: bool,
    expr: Box<Expr>,
}

#[derive(Debug, PartialEq)]
struct BinOpExpr {
    lhs: Box<Expr>,
    bin_op: BinOp,
    rhs: Box<Expr>,
}

#[derive(StrEnum, Debug, PartialEq)]
enum BinOp {
    /// Arithmetic or logical operators
    #[strenum("+")]
    Plus,

    #[strenum("-")]
    Minus,

    #[strenum("*")]
    Star,

    #[strenum("/")]
    Slash,

    #[strenum("%")]
    Percent,

    #[strenum("^")]
    Caret,

    #[strenum("&")]
    And,

    #[strenum("|")]
    Or,

    #[strenum("<<")]
    Shl,

    #[strenum(">>")]
    Shr,

    /// Lazy boolean operators
    #[strenum("&&")]
    AndAnd,

    #[strenum("||")]
    OrOr,

    /// Type cast operator
    As,

    /// Compound assignment operators
    #[strenum("+=")]
    PlusEq,

    #[strenum("-=")]
    MinusEq,

    #[strenum("*=")]
    StarEq,

    #[strenum("/=")]
    SlashEq,

    #[strenum("%=")]
    PercentEq,

    #[strenum("^=")]
    CaretEq,

    #[strenum("&=")]
    AndEq,

    #[strenum("|=")]
    OrEq,

    #[strenum("<<=")]
    ShlEq,

    #[strenum(">>=")]
    ShrEq,

    /// Assignment operators
    #[strenum("=")]
    Eq,

    /// Comparison operators
    #[strenum("==")]
    EqEq,

    #[strenum("!=")]
    Ne,

    #[strenum(">")]
    Gt,

    #[strenum("<")]
    Lt,

    #[strenum(">=")]
    Ge,

    #[strenum("<=")]
    Le,

    /// Range inclusive operators
    #[strenum("..=")]
    DotDotEq,
}

/// GroupExpr -> `(` Expr `)`
#[derive(Debug, PartialEq)]
struct GroupExpr(Box<Expr>);

#[derive(Debug, PartialEq)]
struct ArrayExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
struct ArrayIndexExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
struct TupleExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
struct TupleIndexExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
struct StructExpr;