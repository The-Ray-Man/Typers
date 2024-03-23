use std::{collections::HashMap, fmt::Display};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::typers::{mathjax::MathJax, parser::AstNode, rules::TypeExpr};

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
            constraints,
            num_constraints,
            bottom,
            self.expr.0.current_rule_str()
        )
    }
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

impl MathJax for Tree {
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
            constraints,
            gamma,
            expr,
            self.expr.0.current_rule_str()
        )
    }
}

#[derive(Debug, Clone)]
pub struct TypeInference {
    typ_num: usize,
    pub constraints: Vec<(TypeExpr, TypeExpr)>,
}

impl TypeInference {
    pub fn new(ast: AstNode) -> Result<(Tree, Vec<(TypeExpr, TypeExpr)>), String> {
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

    pub fn new_typ(&mut self) -> TypeExpr {
        let t = TypeExpr::Var(self.typ_num);
        self.typ_num += 1;
        t
    }

    pub fn add_constraint(&mut self, a: &TypeExpr, b: &TypeExpr) {
        self.constraints.push((a.clone(), b.clone()));
    }

    pub fn build_tree(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        let my_ast = ast.clone();
        match my_ast {
            AstNode::Var(_var) => Ok(self.build_var(ast, gamma, t)?),
            AstNode::Abs { var: _, body: _ } => Ok(self.build_abs(ast, gamma, t)?),
            AstNode::App { fun: _, arg: _ } => Ok(self.build_app(ast, gamma, t)?),
            AstNode::IsZero(_expr) => Ok(self.build_zero(ast, gamma, t)?),
            AstNode::Int(_int) => Ok(self.build_int(ast, gamma, t)?),
            AstNode::True => Ok(self.build_bool(ast, gamma, t)?),
            AstNode::False => Ok(self.build_bool(ast, gamma, t)?),
            AstNode::Binop { lhs: _, op: _, rhs: _ } => Ok(self.build_binop(ast, gamma, t)?),
            AstNode::IfThenElse { cond: _, then: _, else_: _ } => {
                Ok(self.build_if_then_else(ast, gamma, t)?)
            }
            AstNode::Tuple { fst: _, snd: _ } => Ok(self.build_tuple(ast, gamma, t)?),
            AstNode::Fst(_expr) => Ok(self.build_fst(ast, gamma, t)?),
            AstNode::Snd(_expr) => Ok(self.build_snd(ast, gamma, t)?),
        }
    }

    fn build_var(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Var(var) => {
                dbg!(&var);
                let type_var = gamma
                    .get(&var.to_string())
                    .ok_or(format!("{} not found!", var).to_string())?;
                self.add_constraint(type_var, &t);
                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![],
                };
                Ok(tree)
            }
            _ => panic!("Expected a variable"),
        }
    }
    fn build_abs(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Abs { var, body } => {
                if let TypeExpr::Function(sigma, tau) = t.clone() {
                    let mut new_gamma = gamma.clone();
                    new_gamma.insert(var, *sigma);
                    let body = self.build_tree(*body, new_gamma, *tau)?;
                    let tree = Tree {
                        gamma,
                        expr: (ast, t),
                        constraints: vec![body],
                    };
                    Ok(tree)
                } else {
                    let sigma = self.new_typ();
                    let tau = self.new_typ();
                    let new_t = TypeExpr::Function(Box::new(sigma.clone()), Box::new(tau.clone()));
                    self.add_constraint(&t, &new_t);
                    let mut new_gamma = gamma.clone();
                    new_gamma.insert(var, sigma);
                    let body = self.build_tree(*body, new_gamma, tau)?;
                    let tree = Tree {
                        gamma,
                        expr: (ast, t),
                        constraints: vec![body],
                    };
                    Ok(tree)
                }
            }
            _ => panic!("Expected an abstraction"),
        }
    }
    fn build_app(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::App { fun, arg } => {
                let sigma = self.new_typ();
                let new_t = TypeExpr::Function(Box::new(sigma.clone()), Box::new(t.clone()));
                let fun = self.build_tree(*fun, gamma.clone(), new_t)?;
                let arg = self.build_tree(*arg, gamma.clone(), sigma.clone())?;
                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![fun, arg],
                };
                Ok(tree)
            }
            _ => panic!("Expected an application"),
        }
    }
    fn build_zero(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::IsZero(expr) => {
                let expr = self.build_tree(*expr, gamma.clone(), TypeExpr::Int)?;
                self.add_constraint(&t, &TypeExpr::Bool);
                let tree = Tree {
                    gamma,
                    expr: (ast, TypeExpr::Bool),
                    constraints: vec![expr],
                };
                Ok(tree)
            }
            _ => panic!("Expected an iszero expression"),
        }
    }
    fn build_int(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast {
            AstNode::Int(_) => {
                self.add_constraint(&t, &TypeExpr::Int);
                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![],
                };
                Ok(tree)
            }
            _ => panic!("Expected an integer"),
        }
    }
    fn build_bool(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast {
            AstNode::True | AstNode::False => {
                self.add_constraint(&t, &TypeExpr::Bool);
                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![],
                };
                Ok(tree)
            }
            _ => panic!("Expected a boolean"),
        }
    }
    fn build_binop(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Binop { lhs, op: _, rhs } => {
                self.add_constraint(&t, &TypeExpr::Int);
                let lhs = self.build_tree(*lhs, gamma.clone(), TypeExpr::Int)?;
                let rhs = self.build_tree(*rhs, gamma.clone(), TypeExpr::Int)?;

                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![lhs, rhs],
                };
                Ok(tree)
            }
            _ => panic!("Expected a binary operation"),
        }
    }
    fn build_if_then_else(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::IfThenElse { cond, then, else_ } => {
                let cond = self.build_tree(*cond, gamma.clone(), TypeExpr::Bool)?;
                let then = self.build_tree(*then, gamma.clone(), t.clone())?;
                let else_ = self.build_tree(*else_, gamma.clone(), t.clone())?;

                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![cond, then, else_],
                };
                Ok(tree)
            }
            _ => panic!("Expected an if-then-else expression"),
        }
    }
    fn build_tuple(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Tuple { fst, snd } => {
                if let TypeExpr::Tuple(a, b) = t.clone() {
                    let fst = self.build_tree(*fst, gamma.clone(), *a)?;
                    let snd = self.build_tree(*snd, gamma.clone(), *b)?;

                    let tree = Tree {
                        gamma,
                        expr: (ast, t),
                        constraints: vec![fst, snd],
                    };
                    Ok(tree)
                } else {
                    let a = self.new_typ();
                    let b = self.new_typ();
                    let new_t = TypeExpr::Tuple(Box::new(a.clone()), Box::new(b.clone()));
                    self.add_constraint(&t, &new_t.clone());
                    let fst = self.build_tree(*fst, gamma.clone(), a)?;
                    let snd = self.build_tree(*snd, gamma.clone(), b)?;

                    let tree = Tree {
                        gamma,
                        expr: (ast, new_t),
                        constraints: vec![fst, snd],
                    };
                    Ok(tree)
                }
            }
            _ => panic!("Expected a tuple"),
        }
    }
    fn build_fst(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Fst(expr) => {
                let b = self.new_typ();
                let new_t = TypeExpr::Tuple(Box::new(t.clone()), Box::new(b));

                let expr = self.build_tree(*expr, gamma.clone(), new_t)?;

                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![expr],
                };
                Ok(tree)
            }
            _ => panic!("Expected a first projection"),
        }
    }
    fn build_snd(
        &mut self,
        ast: AstNode,
        gamma: HashMap<String, TypeExpr>,
        t: TypeExpr,
    ) -> Result<Tree, String> {
        match ast.clone() {
            AstNode::Snd(expr) => {
                let a = self.new_typ();
                let new_t = TypeExpr::Tuple(Box::new(a), Box::new(t.clone()));

                let expr = self.build_tree(*expr, gamma.clone(), new_t)?;

                let tree = Tree {
                    gamma,
                    expr: (ast, t),
                    constraints: vec![expr],
                };
                Ok(tree)
            }
            _ => panic!("Expected a second projection"),
        }
    }
}
