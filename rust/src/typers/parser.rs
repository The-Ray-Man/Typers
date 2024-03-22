use core::fmt;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use wasm_bindgen::prelude::wasm_bindgen;

use super::mathjax::MathJax;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Mult,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Plus => write!(f, "+"),
            BinOp::Mult => write!(f, "*"),
        }
 
   }
}


#[derive(Debug, Clone)]
pub enum AstNode {
    Var(String),
    Abs {
        var: String,
        body: Box<AstNode>,
    },
    App {
        fun: Box<AstNode>,
        arg: Box<AstNode>,
    },
    IsZero(Box<AstNode>),
    Int(i32),
    True,
    False,
    Binop {
        op: BinOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    IfThenElse {
        cond: Box<AstNode>,
        then: Box<AstNode>,
        else_: Box<AstNode>,
    },
    Tuple {
        fst: Box<AstNode>,
        snd: Box<AstNode>,
    },
    Fst(Box<AstNode>),
    Snd(Box<AstNode>),
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstNode::Var(var) => write!(f, "{}", var),
            AstNode::Abs { var, body } => write!(f, "\\{} -> {}", var, body),
            AstNode::App { fun, arg } => write!(f, "({} {})", fun, arg),
            AstNode::IsZero(expr) => write!(f, "iszero {}", expr),
            AstNode::Int(int) => write!(f, "{}", int),
            AstNode::True => write!(f, "true"),
            AstNode::False => write!(f, "false"),
            AstNode::Binop { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            AstNode::IfThenElse { cond, then, else_ } => write!(f, "if {} then {} else {}", cond, then, else_),
            AstNode::Tuple { fst, snd } => write!(f, "({}, {})", fst, snd),
            AstNode::Fst(expr) => write!(f, "fst {}", expr),
            AstNode::Snd(expr) => write!(f, "snd {}", expr),
        }
    }
}

impl MathJax for AstNode{
    fn to_mathjax(&self) -> String {
        match self {
            AstNode::Var(var) => var.clone(),
            AstNode::Abs { var, body } => format!("\\lambda {} \\ . \\ {}", var, body.to_mathjax()),
            AstNode::App { fun, arg } => format!("({} \\ {})", fun.to_mathjax(), arg.to_mathjax()),
            AstNode::IsZero(expr) => format!("\\mathsf{{iszero}} {}", expr.to_mathjax()),
            AstNode::Int(int) => int.to_string(),
            AstNode::True => "\\mathsf{{true}}".to_string(),
            AstNode::False => "\\mathsf{{false}}".to_string(),
            AstNode::Binop { op, lhs, rhs } => format!("({} {} {})", lhs.to_mathjax(), op, rhs.to_mathjax()),
            AstNode::IfThenElse { cond, then, else_ } => format!("\\mathsf{{if}} \\ {} \\ \\mathsf{{then}} \\ {} \\ \\mathsf{{else}} \\ {}", cond.to_mathjax(), then.to_mathjax(), else_.to_mathjax()),
            AstNode::Tuple { fst, snd } => format!("({},\\ {})", fst.to_mathjax(), snd.to_mathjax()),
            AstNode::Fst(expr) => format!("\\mathsf{{fst}} \\ {}", expr.to_mathjax()),
            AstNode::Snd(expr) => format!("\\mathsf{{snd}} \\ {}", expr.to_mathjax()),
        }
    }
}

impl AstNode {
    pub fn current_rule_str(&self) -> String {
        let res = match self {
            AstNode::Var(_) => "Var",
            AstNode::Abs { .. } => "Abs",
            AstNode::App { .. } => "App",
            AstNode::IsZero(_) => "iszero",
            AstNode::Int(_) => "Int",
            AstNode::True => "True",
            AstNode::False => "False",
            AstNode::Binop { .. } => "BinOp",
            AstNode::IfThenElse { .. } => "if",
            AstNode::Tuple { .. } => "tuple",
            AstNode::Fst(_) => "fst",
            AstNode::Snd(_) => "snd",
        };
        res.to_string()
    }
}


#[derive(pest_derive::Parser)]
#[grammar = "./typers/miniHaskell.pest"]
pub struct MiniHaskellParser;

impl MiniHaskellParser {
    pub fn parse_str(input: &str) -> Result<Pair<'_, Rule>, String> {
        let parsed = Self::parse(Rule::expr, input);

        let mut parsed = parsed.map_err(|e| format!("{}", e))?;

        let first_pair = parsed.next().ok_or("no first pair".to_string())?;

        Ok(first_pair)
    }

    pub fn build_ast_(pair: Pair<Rule>) -> Result<AstNode, String> {
        match pair.as_rule() {
            Rule::var => Ok(Self::build_ast_var(pair)?),
            Rule::abs => Ok(Self::build_ast_abs(pair)?),
            Rule::app => Ok(Self::build_ast_app(pair)?),
            Rule::iszero => Ok(Self::build_ast_iszero(pair)?),
            Rule::int => Ok(Self::build_ast_int(pair)?),
            Rule::boolean => Ok(Self::build_ast_boolean(pair)?),
            Rule::binOp => Ok(Self::build_ast_binop(pair)?),
            Rule::ifThenElse => Ok(Self::build_ast_if_then_else(pair)?),
            Rule::tuple => Ok(Self::build_ast_tuple(pair)?),
            Rule::fst => Ok(Self::build_ast_fst(pair)?),
            Rule::snd => Ok(Self::build_ast_snd(pair)?),
            e => {
                dbg!(e);
                Err("rule not implemented".to_string())
            }
        }
    }

    pub fn build_ast_var(pair: Pair<Rule>) -> Result<AstNode, String> {
        Ok(AstNode::Var(pair.as_str().to_string()))
    }
    
    pub fn build_ast_abs(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let var = pairs.next().ok_or("no var".to_string())?;
        let body = pairs.next().ok_or("no body".to_string())?;
    
        let var = var.as_str().to_string();
        let body = Self::build_ast_(body)?;
    
        Ok(AstNode::Abs {
            var,
            body: Box::new(body),
        })
    }

    pub fn build_ast_app(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let fun = pairs.next().ok_or("no fun".to_string())?;
        let arg = pairs.next().ok_or("no arg".to_string())?;

        let fun = Self::build_ast_(fun)?;
        let arg = Self::build_ast_(arg)?;

        Ok(AstNode::App {
            fun: Box::new(fun),
            arg: Box::new(arg),
        })
    }

    pub fn build_ast_iszero(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast_(expr)?;
        Ok(AstNode::IsZero(Box::new(expr)))
    }

    pub fn build_ast_int(pair: Pair<Rule>) -> Result<AstNode, String> {
        let int = pair.as_str().parse::<i32>().map_err(|e| format!("{}", e))?;
        Ok(AstNode::Int(int))
    }

    pub fn build_ast_boolean(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let pair = pairs.next().ok_or("no boolean".to_string())?;

        match pair.as_rule() {
            Rule::true_ => Ok(AstNode::True),
            Rule::false_ => Ok(AstNode::False),
            e => Err(format!("expected boolean but got: {:?}", e)),
        }
    }

    pub fn build_ast_op(pair: Pair<Rule>) -> Result<BinOp, String> {
        match pair.as_rule() {
            Rule::add => Ok(BinOp::Plus),
            Rule::mult => Ok(BinOp::Mult),
            e => Err(format!("expected binop but got: {:?}", e)),
        }
    }

    pub fn build_ast_binop(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();

        let lhs = pairs.next().ok_or("no lhs".to_string())?;
        let binop = pairs.next().ok_or("no binop".to_string())?;
        let rhs = pairs.next().ok_or("no rhs".to_string())?;

        dbg!(&lhs, &binop, &rhs);

        let lhs = Self::build_ast_(lhs)?;
        let op = Self::build_ast_op(binop)?;
        let rhs = Self::build_ast_(rhs)?;
        Ok(AstNode::Binop {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    pub fn build_ast_if_then_else(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let cond = pairs.next().ok_or("no cond".to_string())?;
        let then = pairs.next().ok_or("no then".to_string())?;
        let else_ = pairs.next().ok_or("no else".to_string())?;

        let cond = Self::build_ast_(cond)?;
        let then = Self::build_ast_(then)?;
        let else_ = Self::build_ast_(else_)?;

        Ok(AstNode::IfThenElse {
            cond: Box::new(cond),
            then: Box::new(then),
            else_: Box::new(else_),
        })
    }

    pub fn build_ast_tuple(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let fst = pairs.next().ok_or("no fst".to_string())?;
        let snd = pairs.next().ok_or("no snd".to_string())?;

        let fst = Self::build_ast_(fst)?;
        let snd = Self::build_ast_(snd)?;

        Ok(AstNode::Tuple {
            fst: Box::new(fst),
            snd: Box::new(snd),
        })
    }

    pub fn build_ast_fst(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast_(expr)?;
        Ok(AstNode::Fst(Box::new(expr)))
    }

    pub fn build_ast_snd(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast_(expr)?;
        Ok(AstNode::Snd(Box::new(expr)))
    }

}
