use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::typers::{parser::AstNode, rules::TypeExpr, utils::mathjax::MathJax};

use super::parser::BinOp;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct TreeTS {
    pub gamma: String,
    pub expr: String,
    pub constraints: Vec<TreeTS>,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub gamma: HashMap<String, TypeExpr>,
    pub expr: (AstNode, TypeExpr),
    pub constraints: Vec<Tree>,
}

impl From<Tree> for TreeTS {
    fn from(val: Tree) -> Self {
        let gamma = val
            .gamma
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_mathjax()))
            .collect::<Vec<String>>()
            .join(", ");
        let expr = format!("{} :: {}", val.expr.0, val.expr.1);
        let constraints = val
            .constraints
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<TreeTS>>();
        TreeTS {
            gamma,
            expr,
            constraints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeInference {
    typ_num: usize,
    pub constraints: Vec<(TypeExpr, TypeExpr)>,
}

impl TypeInference {
    // Transform an AST into a tree and a list of constraints.
    pub fn infer(ast: AstNode) -> Result<(Tree, Vec<(TypeExpr, TypeExpr)>), String> {
        let gamma = HashMap::new();
        let constraints = Vec::<(TypeExpr, TypeExpr)>::new();
        let mut type_inference = TypeInference {
            typ_num: 0,
            constraints,
        };
        let start_t = type_inference.new_typ();
        let res = type_inference.build_tree(ast, gamma, start_t)?;
        Ok((res, type_inference.constraints))
    }
    // gives a new type variable with a fresh number.
    pub fn new_typ(&mut self) -> TypeExpr {
        let t = TypeExpr::Var(self.typ_num);
        self.typ_num += 1;
        t
    }

    // Add the constraint a = b to the constraint list.
    pub fn add_constraint(&mut self, a: &TypeExpr, b: &TypeExpr) {
        self.constraints.push((a.clone(), b.clone()));
    }

    // Build a tree with types from an AST. It collects the type constraints.
    pub fn build_tree(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        let my_ast = ast.clone();
        match my_ast {
            AstNode::Var(var) => Ok(self.build_var(var, gamma, t)?),
            AstNode::Abs { var, body } => Ok(self.build_abs(var, *body, gamma, t)?),
            AstNode::App { fun, arg } => Ok(self.build_app(*fun, *arg, gamma, t)?),
            AstNode::IsZero(expr) => Ok(self.build_zero(*expr, gamma, t)?),
            AstNode::Int(num) => Ok(self.build_int(num, gamma, t)?),
            AstNode::True => Ok(self.build_bool(true, gamma, t)?),
            AstNode::False => Ok(self.build_bool(false, gamma, t)?),
            AstNode::BinOp { lhs, op, rhs } => Ok(self.build_binop(*lhs, op, *rhs, gamma, t)?),
            AstNode::IfThenElse { cond, then, else_ } => {
                Ok(self.build_if_then_else(*cond, *then, *else_, gamma, t)?)
            }
            AstNode::Tuple { fst, snd } => Ok(self.build_tuple(*fst, *snd, gamma, t)?),
            AstNode::Fst(expr) => Ok(self.build_fst(*expr, gamma, t)?),
            AstNode::Snd(expr) => Ok(self.build_snd(*expr, gamma, t)?),
        }
    }

    // Build a tree, starting from a variable rule.
    fn build_var(
        &mut self,
        var: String,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        // Check if the variable is in the gamma. If it is not, the tree is invalid and a error is returned.
        dbg!("build var");
        let type_var = gamma
            .get(&var.to_string())
            .ok_or(format!("{} not found!", var).to_string())?;

        self.add_constraint(type_var, &t);

        Ok(Tree {
            gamma,
            expr: (AstNode::Var(var), t),
            constraints: Vec::<Tree>::new(),
        })
    }

    // Build a tree, starting from an abstraction rule.
    fn build_abs(
        &mut self,
        var: String,
        body: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        // Check if the type of the expression has already the function form.
        dbg!("build abs");
        if let TypeExpr::Function(sigma, tau) = t.clone() {
            let mut new_gamma = gamma.clone();
            new_gamma.insert(var.clone(), *sigma);
            let body_tree = self.build_tree(body.clone(), new_gamma, *tau)?;
            Ok(Tree {
                gamma,
                expr: (
                    AstNode::Abs {
                        var,
                        body: Box::new(body),
                    },
                    t,
                ),
                constraints: vec![body_tree],
            })
        } else {
            // The type of the expression is not a function, so we need to create a new function type and add a constraint.
            let sigma = self.new_typ();
            let tau = self.new_typ();
            let new_t = TypeExpr::Function(Box::new(sigma.clone()), Box::new(tau.clone()));
            self.add_constraint(&t, &new_t);
            let mut new_gamma = gamma.clone();
            new_gamma.insert(var.clone(), sigma);
            let body_tree = self.build_tree(body.clone(), new_gamma, tau)?;
            Ok(Tree {
                gamma,
                expr: (
                    AstNode::Abs {
                        var,
                        body: Box::new(body),
                    },
                    t,
                ),
                constraints: vec![body_tree],
            })
        }
    }
    // Build a tree, starting from an application rule.
    fn build_app(
        &mut self,
        fun: AstNode,
        arg: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build app");
        let sigma = self.new_typ();
        let new_t = TypeExpr::Function(Box::new(sigma.clone()), Box::new(t.clone()));
        let fun_tree = self.build_tree(fun.clone(), gamma.clone(), new_t)?;
        let arg_tree = self.build_tree(arg.clone(), gamma.clone(), sigma.clone())?;
        Ok(Tree {
            gamma,
            expr: (
                AstNode::App {
                    fun: Box::new(fun),
                    arg: Box::new(arg),
                },
                t,
            ),
            constraints: vec![fun_tree, arg_tree],
        })
    }

    // Build a tree, starting from an iszero rule.
    fn build_zero(
        &mut self,
        expr: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build zero");
        let expr_tree = self.build_tree(expr.clone(), gamma.clone(), TypeExpr::Int)?;
        self.add_constraint(&t, &TypeExpr::Bool);
        Ok(Tree {
            gamma,
            expr: (AstNode::IsZero(Box::new(expr)), TypeExpr::Bool),
            constraints: vec![expr_tree],
        })
    }
    // Build a tree, starting from an integer rule.
    fn build_int(
        &mut self,
        num: i32,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build int");
        self.add_constraint(&t, &TypeExpr::Int);
        Ok(Tree {
            gamma,
            expr: (AstNode::Int(num), t),
            constraints: vec![],
        })
    }
    // Build a tree, starting from a boolean rule.
    fn build_bool(
        &mut self,
        value: bool,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build bool");
        self.add_constraint(&t, &TypeExpr::Bool);
        Ok(Tree {
            gamma,
            expr: (if value { AstNode::True } else { AstNode::False }, t),
            constraints: vec![],
        })
    }

    // Build a tree, starting from a binary operation rule.
    fn build_binop(
        &mut self,
        lhs: AstNode,
        op: BinOp,
        rhs: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build binop");
        self.add_constraint(&t, &TypeExpr::Int);
        let lhs_tree = self.build_tree(lhs.clone(), gamma.clone(), TypeExpr::Int)?;
        let rhs_tree = self.build_tree(rhs.clone(), gamma.clone(), TypeExpr::Int)?;

        Ok(Tree {
            gamma,
            expr: (
                AstNode::BinOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                t,
            ),
            constraints: vec![lhs_tree, rhs_tree],
        })
    }

    // Build a tree, starting from an if-then-else rule.
    fn build_if_then_else(
        &mut self,
        cond: AstNode,
        then: AstNode,
        else_: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build if then else");
        let cond_tree = self.build_tree(cond.clone(), gamma.clone(), TypeExpr::Bool)?;
        let then_tree = self.build_tree(then.clone(), gamma.clone(), t.clone())?;
        let else_tree = self.build_tree(else_.clone(), gamma.clone(), t.clone())?;

        Ok(Tree {
            gamma,
            expr: (
                AstNode::IfThenElse {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    else_: Box::new(else_),
                },
                t,
            ),
            constraints: vec![cond_tree, then_tree, else_tree],
        })
    }

    // Build a tree, starting from a tuple rule.
    fn build_tuple(
        &mut self,
        fst: AstNode,
        snd: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build tuple");
        // Check if the type of the expression has already the tuple form.
        if let TypeExpr::Tuple(a, b) = t.clone() {
            let fst_tree = self.build_tree(fst.clone(), gamma.clone(), *a)?;
            let snd_tree = self.build_tree(snd.clone(), gamma.clone(), *b)?;

            Ok(Tree {
                gamma,
                expr: (
                    AstNode::Tuple {
                        fst: Box::new(fst),
                        snd: Box::new(snd),
                    },
                    t,
                ),
                constraints: vec![fst_tree, snd_tree],
            })
        } else {
            // The type of the expression is not a tuple, so we need to create a new tuple type and add a constraint.
            let a = self.new_typ();
            let b = self.new_typ();
            let new_t = TypeExpr::Tuple(Box::new(a.clone()), Box::new(b.clone()));
            self.add_constraint(&t, &new_t.clone());
            let fst_tree = self.build_tree(fst.clone(), gamma.clone(), a)?;
            let snd_tree = self.build_tree(snd.clone(), gamma.clone(), b)?;

            Ok(Tree {
                gamma,
                expr: (
                    AstNode::Tuple {
                        fst: Box::new(fst),
                        snd: Box::new(snd),
                    },
                    new_t,
                ),
                constraints: vec![fst_tree, snd_tree],
            })
        }
    }

    // Build a tree, starting from a first projection rule.
    fn build_fst(
        &mut self,
        expr: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build fst");
        let b = self.new_typ();
        let new_t = TypeExpr::Tuple(Box::new(t.clone()), Box::new(b));

        let expr_tree = self.build_tree(expr.clone(), gamma.clone(), new_t)?;

        Ok(Tree {
            gamma,
            expr: (AstNode::Fst(Box::new(expr)), t),
            constraints: vec![expr_tree],
        })
    }

    // Build a tree, starting from a second projection rule.
    fn build_snd(
        &mut self,
        expr: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        dbg!("build snd");
        let a = self.new_typ();
        let new_t = TypeExpr::Tuple(Box::new(a), Box::new(t.clone()));

        let expr_tree = self.build_tree(expr.clone(), gamma.clone(), new_t)?;

        let tree = Tree {
            gamma,
            expr: (AstNode::Snd(Box::new(expr)), t),
            constraints: vec![expr_tree],
        };
        Ok(tree)
    }
}
