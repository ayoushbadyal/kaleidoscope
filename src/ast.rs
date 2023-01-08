/*
  function type Normal,Urinary,Binary
*/
use crate::token::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NumberExprAst {
    pub val: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExprAst {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExprAst {
    pub op: String,
    pub lhs: Box<ExprAst>,
    pub rhs: Box<ExprAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExprAst {
    pub callee: String,
    pub args: Vec<ExprAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixAST {
    pub op: String,
    pub expr: Box<ExprAst>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExprAst {
    NumberExpr(NumberExprAst),
    VariableExpr(VariableExprAst),
    PreFix(PrefixAST),
    BinaryExpr(BinaryExprAst),
    Stmt(Stmt),
    CallExpr(CallExprAst),
    Func(FunctionAst),
    ExternNode(ExternNodeAst),
    ForLoopBlock(ForIn),
    MultipleScopeVars(VarMultipleVars), // rename
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assignment(Token, Box<ExprAst>),
    Ifelse(ConditionalBl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalBl {
    pub condi: Box<ExprAst>,
    pub body: Vec<ExprAst>,
    pub else_if_chain: Vec<(ExprAst, Vec<ExprAst>)>,
    pub else_co: Vec<ExprAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrototypeAst {
    pub name: String,
    pub kind: FunctionType,
    pub args: Vec<ExprAst>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FunctionType {
    Normal,
    UnaryOp(String),
    BinaryOp(String, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAst {
    pub proto: PrototypeAst,
    pub body: Box<Vec<ExprAst>>,
    // pub ret_stmt: Option<Box<ExprAst>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarMultipleVars {
    pub vars: Vec<ExprAst>,
    pub body: Vec<ExprAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForIn {
    // pub ret_stmt: Option<Box<ExprAst>>,
    pub init: Box<ExprAst>,
    pub cond: Box<ExprAst>,
    pub inc_val: f64,
    pub body: Vec<ExprAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternNodeAst {
    pub body: Box<ExprAst>,
}

#[derive(Debug)]
pub enum Ast {
    FAst(FunctionAst),
    PAst(PrototypeAst),
    EAst(ExprAst),
}

impl Display for ExprAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprAst::NumberExpr(NumberExprAst { val }) => write!(f, "{}", *val),
            ExprAst::VariableExpr(VariableExprAst { name }) => write!(f, "{}", *name),
            ExprAst::PreFix(a) => write!(f, "{}({})", a.op, a.expr),
            ExprAst::BinaryExpr(BinaryExprAst { op, lhs, rhs }) => {
                write!(f, "({}{}{})", lhs, op, rhs)
            }
            ExprAst::Stmt(Stmt::Assignment(l, r)) => {
                write!(f, "{}={}", String::from(l.clone()), r)
            }
            ExprAst::Stmt(Stmt::Ifelse(a)) => {
                write!(f, "{}", a)
            }
            ExprAst::ForLoopBlock(a) => {
                let mut d = String::new();
                for x in a.body.iter() {
                    d.push_str(format!("\n\t\t{}\n\t", x).as_str());
                }
                write!(f, "for {},{},{} in{}", a.init, a.cond, a.inc_val, d)
            }
            ExprAst::CallExpr(CallExprAst { callee, args }) => {
                write!(
                    f,
                    "{}({})",
                    callee,
                    String::from_iter(args.clone().into_iter())
                )
            }
            ExprAst::Func(FunctionAst { proto, body }) => {
                let c = String::from_iter(proto.clone().args.into_iter());
                // let d = String::from_iter(body.clone().into_iter());
                let mut d = String::new();
                for x in body.iter() {
                    d.push_str(format!("\n\t{}\n\t", x).as_str());
                }
                write!(f, "def {}({}){}", proto.clone().name, c, d)
            }
            ExprAst::ExternNode(a) => write!(
                f,
                "{}
                ",
                a.body
            ),
            ExprAst::MultipleScopeVars(a) => {
                write!(
                    f,
                    "var {} in\n\t\t{}",
                    String::from_iter(a.clone().vars),
                    String::from_iter(a.clone().body)
                )
            }
        }
    }
}

//TODO
impl Display for ConditionalBl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if {} \n\t\t{}\n\telse\n\t\t{}\n",
            self.condi.clone(),
            String::from_iter(self.body.clone()),
            String::from_iter(self.else_co.clone()),
            // self.else_co.clone()
        )
    }
}

impl FromIterator<ExprAst> for String {
    fn from_iter<T: IntoIterator<Item = ExprAst>>(iter: T) -> Self {
        let mut s = String::new();
        for x in iter.into_iter() {
            s.push_str(format!(",{}", x).as_str());
        }
        let ss = s.as_str().replacen(",", "", 1);
        ss
    }
}

impl From<Token> for String {
    // @@@: is this the Right way
    fn from(se: Token) -> Self {
        let mut s = String::new();
        match se {
            Token::Identifier(a) => {
                s = a;
            }
            Token::Char(a) => {
                s = a.into();
            }
            _ => {}
        }
        s
    }
}
impl From<Token> for f64 {
    // @@@: is this the Right way
    fn from(se: Token) -> Self {
        let mut s = 0f64;
        match se {
            Token::Number(a) => {
                s += a;
            }
            _ => {}
        }
        s
    }
}
