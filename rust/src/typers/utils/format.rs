use std::fmt::{Display, Formatter, Result};

use crate::typers::{
    parser::{AstNode, BinOp},
    rules::{RuleExpr, TypeExpr},
    tree::Tree,
};

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BinOp::Plus => write!(f, "+"),
            BinOp::Mult => write!(f, "*"),
        }
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AstNode::Var(var) => write!(f, "{}", var),
            AstNode::Abs { var, body } => write!(f, "\\{} -> {}", var, body),
            AstNode::App { fun, arg } => write!(f, "({} {})", fun, arg),
            AstNode::IsZero(expr) => write!(f, "iszero {}", expr),
            AstNode::Int(int) => write!(f, "{}", int),
            AstNode::True => write!(f, "true"),
            AstNode::False => write!(f, "false"),
            AstNode::BinOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            AstNode::IfThenElse { cond, then, else_ } => {
                write!(f, "if {} then {} else {}", cond, then, else_)
            }
            AstNode::Tuple { fst, snd } => write!(f, "({}, {})", fst, snd),
            AstNode::Fst(expr) => write!(f, "fst {}", expr),
            AstNode::Snd(expr) => write!(f, "snd {}", expr),
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gamma = self
            .gamma
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(", ");
        let bottom = format!("{} ⊢ {} :: {}", gamma, self.expr.0, self.expr.1);
        let num_constraints = self.constraints.len();
        let constraints = self
            .constraints
            .iter()
            .map(|a| format!("{}", a))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "{}\nrule(n: {}, \"{}\", label:\"{}\"),",
            constraints, num_constraints, bottom, self.expr.0
        )
    }
}

impl From<AstNode> for String {
    fn from(val: AstNode) -> Self {
        let res = match val {
            AstNode::Var(_) => "Var",
            AstNode::Abs { .. } => "Abs",
            AstNode::App { .. } => "App",
            AstNode::IsZero(_) => "iszero",
            AstNode::Int(_) => "Int",
            AstNode::True => "True",
            AstNode::False => "False",
            AstNode::BinOp { .. } => "BinOp",
            AstNode::IfThenElse { .. } => "if",
            AstNode::Tuple { .. } => "tuple",
            AstNode::Fst(_) => "fst",
            AstNode::Snd(_) => "snd",
        };
        res.to_string()
    }
}

impl Display for TypeExpr {
    /// Recursively display the type expression, wraps some variants in parenthesis
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TypeExpr::Function(t1, t2) => {
                let t1_out = if t1.needs_wrapping() {
                    format!("({t1})")
                } else {
                    format!("{t1}")
                };
                // since -> is right associative, no need to put parenthesis around the right part
                write!(f, "{t1_out} -> {t2}")
            }
            TypeExpr::Tuple(t1, t2) => {
                let t1_out = if t1.needs_wrapping() {
                    format!("({t1})")
                } else {
                    format!("{t1}")
                };
                let t2_out = if t2.needs_wrapping() {
                    format!("({t2})")
                } else {
                    format!("{t2}")
                };
                write!(f, "({t1_out}, {t2_out})")
            }
            TypeExpr::Var(x) => write!(f, "t{x}"),
            TypeExpr::Bool => write!(f, "Bool"),
            TypeExpr::Int => write!(f, "Int"),
        }
    }
}

impl Display for RuleExpr {
    /// Assumes that variables are being displayed in the form `t0` for variable ID `0`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t{} = {}", self.var, self.rhs)
    }
}
