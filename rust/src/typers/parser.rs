
use pest::{iterators::Pair, Parser};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents binary operators.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Mult,
}

/// Represents an abstract syntax tree node.
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
    BinOp {
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

/// Parser for the MiniHaskell language.
#[derive(pest_derive::Parser)]
#[grammar = "./typers/miniHaskell.pest"]
pub struct MiniHaskellParser;

impl MiniHaskellParser {
    /// Parses a string into a `Pair` representing the root rule of the grammar.
    pub fn parse_str(input: &str) -> Result<Pair<'_, Rule>, String> {
        // Parse the input string
        let mut parsed = Self::parse(Rule::expr, input).map_err(|e| format!("{}", e))?;

        let first_pair = parsed.next().ok_or("no first pair".to_string())?;

        Ok(first_pair)
    }

    /// Builds an abstract syntax tree from a `Pair` representing a rule.
    pub fn build_ast(pair: Pair<Rule>) -> Result<AstNode, String> {
        match pair.as_rule() {
            Rule::var => Ok(Self::build_ast_var(pair)?),
            Rule::abs => Ok(Self::build_ast_abs(pair)?),
            Rule::app => Ok(Self::build_ast_app(pair)?),
            Rule::isZero => Ok(Self::build_ast_iszero(pair)?),
            Rule::int => Ok(Self::build_ast_int(pair)?),
            Rule::boolean => Ok(Self::build_ast_boolean(pair)?),
            Rule::binOp => Ok(Self::build_ast_binop(pair)?),
            Rule::ifThenElse => Ok(Self::build_ast_if_then_else(pair)?),
            Rule::tuple => Ok(Self::build_ast_tuple(pair)?),
            Rule::fst => Ok(Self::build_ast_fst(pair)?),
            Rule::snd => Ok(Self::build_ast_snd(pair)?),
            e => Err(format!("unexpected rule: {:?}", e)),
        }
    }

    /// Builds an abstract syntax tree node for a variable.
    pub fn build_ast_var(pair: Pair<Rule>) -> Result<AstNode, String> {
        Ok(AstNode::Var(pair.as_str().to_string()))
    }

    /// Builds an abstract syntax tree node for an abstraction.
    pub fn build_ast_abs(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let var = pairs.next().ok_or("no var".to_string())?;
        let body = pairs.next().ok_or("no body".to_string())?;

        let var = var.as_str().to_string();
        let body = Self::build_ast(body)?;

        Ok(AstNode::Abs {
            var,
            body: Box::new(body),
        })
    }

    /// Builds an abstract syntax tree node for an application.
    pub fn build_ast_app(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let fun = pairs.next().ok_or("no fun".to_string())?;
        let arg = pairs.next().ok_or("no arg".to_string())?;

        let fun = Self::build_ast(fun)?;
        let arg = Self::build_ast(arg)?;

        Ok(AstNode::App {
            fun: Box::new(fun),
            arg: Box::new(arg),
        })
    }

    /// Builds an abstract syntax tree node for an iszero expression.
    pub fn build_ast_iszero(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast(expr)?;
        Ok(AstNode::IsZero(Box::new(expr)))
    }

    /// Builds an abstract syntax tree node for an integer.
    pub fn build_ast_int(pair: Pair<Rule>) -> Result<AstNode, String> {
        let int = pair.as_str().parse::<i32>().map_err(|e| format!("{}", e))?;
        Ok(AstNode::Int(int))
    }

    /// Builds an abstract syntax tree node for a boolean.
    pub fn build_ast_boolean(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let pair = pairs.next().ok_or("no boolean".to_string())?;

        match pair.as_rule() {
            Rule::true_ => Ok(AstNode::True),
            Rule::false_ => Ok(AstNode::False),
            e => Err(format!("expected boolean but got: {:?}", e)),
        }
    }

    /// Builds a binary operator from a `Pair` representing a binOp rule.
    pub fn build_ast_op(pair: Pair<Rule>) -> Result<BinOp, String> {
        match pair.as_rule() {
            Rule::add => Ok(BinOp::Plus),
            Rule::mult => Ok(BinOp::Mult),
            e => Err(format!("expected binop but got: {:?}", e)),
        }
    }

    /// Builds an abstract syntax tree node for a binary operation.
    pub fn build_ast_binop(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();

        let lhs = pairs.next().ok_or("no lhs".to_string())?;
        let binop = pairs.next().ok_or("no binop".to_string())?;
        let rhs = pairs.next().ok_or("no rhs".to_string())?;

        dbg!(&lhs, &binop, &rhs);

        let lhs = Self::build_ast(lhs)?;
        let op = Self::build_ast_op(binop)?;
        let rhs = Self::build_ast(rhs)?;
        Ok(AstNode::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    /// Builds an abstract syntax tree node for an if-then-else expression.
    pub fn build_ast_if_then_else(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let cond = pairs.next().ok_or("no cond".to_string())?;
        let then = pairs.next().ok_or("no then".to_string())?;
        let else_ = pairs.next().ok_or("no else".to_string())?;

        let cond = Self::build_ast(cond)?;
        let then = Self::build_ast(then)?;
        let else_ = Self::build_ast(else_)?;

        Ok(AstNode::IfThenElse {
            cond: Box::new(cond),
            then: Box::new(then),
            else_: Box::new(else_),
        })
    }

    /// Builds an abstract syntax tree node for a tuple.
    pub fn build_ast_tuple(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let fst = pairs.next().ok_or("no fst".to_string())?;
        let snd = pairs.next().ok_or("no snd".to_string())?;

        let fst = Self::build_ast(fst)?;
        let snd = Self::build_ast(snd)?;

        Ok(AstNode::Tuple {
            fst: Box::new(fst),
            snd: Box::new(snd),
        })
    }

    /// Builds an abstract syntax tree node for the `fst` operation.
    pub fn build_ast_fst(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast(expr)?;
        Ok(AstNode::Fst(Box::new(expr)))
    }

    /// Builds an abstract syntax tree node for the `snd` operation.
    pub fn build_ast_snd(pair: Pair<Rule>) -> Result<AstNode, String> {
        let mut pairs = pair.into_inner();
        let expr = pairs.next().ok_or("no expr".to_string())?;
        let expr = Self::build_ast(expr)?;
        Ok(AstNode::Snd(Box::new(expr)))
    }
}
