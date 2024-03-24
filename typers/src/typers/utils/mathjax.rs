use crate::typers::{
    parser::AstNode,
    rules::{RuleExpr, TypeExpr},
    tree::Tree,
};

pub trait MathJax {
    fn to_mathjax(&self) -> String;
}

impl MathJax for AstNode {
    //translates the AST to a string that can be rendered by MathJax
    fn to_mathjax(&self) -> String {
        match self {
            AstNode::Var(var) => var.clone(),
            AstNode::Abs { var, body } => format!("\\lambda {} \\ . \\ {}", var, body.to_mathjax()),
            AstNode::App { fun, arg } => format!("({} \\ {})", fun.to_mathjax(), arg.to_mathjax()),
            AstNode::IsZero(expr) => format!("\\mathsf{{iszero}} \\ {}", expr.to_mathjax()),
            AstNode::Int(int) => int.to_string(),
            AstNode::True => "\\mathsf{{true}}".to_string(),
            AstNode::False => "\\mathsf{{false}}".to_string(),
            AstNode::BinOp { op, lhs, rhs } => {
                format!("({} {} {})", lhs.to_mathjax(), op, rhs.to_mathjax())
            }
            AstNode::IfThenElse { cond, then, else_ } => format!(
                "\\mathsf{{if}} \\ {} \\ \\mathsf{{then}} \\ {} \\ \\mathsf{{else}} \\ {}",
                cond.to_mathjax(),
                then.to_mathjax(),
                else_.to_mathjax()
            ),
            AstNode::Tuple { fst, snd } => {
                format!("({},\\ {})", fst.to_mathjax(), snd.to_mathjax())
            }
            AstNode::Fst(expr) => format!("\\mathsf{{fst}} \\ {}", expr.to_mathjax()),
            AstNode::Snd(expr) => format!("\\mathsf{{snd}} \\ {}", expr.to_mathjax()),
        }
    }
}

impl MathJax for TypeExpr {
    // translates the type expression to a string that can be rendered by MathJax
    fn to_mathjax(&self) -> String {
        match self {
            TypeExpr::Function(left, right) => {
                format!("({} \\to {})", left.to_mathjax(), right.to_mathjax())
            }
            TypeExpr::Tuple(left, right) => {
                format!("({}, {})", left.to_mathjax(), right.to_mathjax())
            }
            TypeExpr::Var(x) => format!("t_{{{}}}", x),
            TypeExpr::Bool => "Bool".to_string(),
            TypeExpr::Int => "Int".to_string(),
        }
    }
}

impl MathJax for Tree {
    // translates the tree to a string that can be rendered by MathJax
    fn to_mathjax(&self) -> String {
        let gamma = self
            .gamma
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_mathjax()))
            .collect::<Vec<String>>()
            .join(", ");
        let expr = format!(
            "{} :: {}",
            self.expr.0.to_mathjax(),
            self.expr.1.to_mathjax()
        );
        let constraints = self
            .constraints
            .iter()
            .map(|a| a.to_mathjax())
            .collect::<Vec<String>>()
            .join("\\qquad");
        format!(
            "\\dfrac{{{}}} {{{} \\vdash {}}} \\textsf{{{}}}",
            constraints, gamma, expr, self.expr.0
        )
    }
}

impl MathJax for RuleExpr {
    // translates the rule expression to a string that can be rendered by MathJax
    fn to_mathjax(&self) -> String {
        format!("t_{{{}}} = {}", self.var, self.rhs.to_mathjax())
    }
}
