use super::pattern::Pattern;
use crate::ast::expr::Expr;
use crate::ast::types::TypeAnnotation;
use crate::ast::stmt::Stmt::ExprStmt;
use crate::ast::item::Item;
use crate::analyser::sym_resolver::VarKind;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Semi,
    Item(Item),
    Let(LetStmt),
    ExprStmt(Expr),
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        ExprStmt(expr)
    }
}

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub pattern: Pattern,
    pub _type: Option<TypeAnnotation>,
    pub expr: Option<Expr>,
}

impl LetStmt {
    pub fn new(pattern: Pattern) -> Self {
        LetStmt {
            pattern,
            _type: None,
            expr: None,
        }
    }

    pub fn _type(mut self, _type: TypeAnnotation) -> Self {
        self._type = Some(_type);
        self
    }

    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }
    
    pub fn is_mut(&self) -> bool {
        match &self.pattern {
            Pattern::Identifier(i) => {
                i.is_mut()
            }
        }
    }
}
