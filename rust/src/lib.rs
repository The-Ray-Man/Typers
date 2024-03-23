mod typers;
mod utils;

use std::{cmp, collections::HashSet};

use typers::{
    mathjax::MathJax,
    parser::{AstNode, MiniHaskellParser},
    rules::RuleExpr,
    solver::solve_constraints,
    tree::{TreeTS, TypeInference},
};

use tsify::Tsify;
use typers::rules::TypeExpr;
use typers::solver::SolutionTS;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Default)]
pub struct Parsed {
    pub parse_error: Option<String>,
    pub build_tree_error: Option<String>,
    pub tree: Option<String>,
    pub constraints_error: Option<String>,
    pub constraints: Option<Vec<String>>,
    pub solution_error: Option<String>,
    pub solution: Option<SolutionTS>,
}

#[wasm_bindgen]
pub fn parse_input(input: &str) -> Parsed {
    let parsed = MiniHaskellParser::parse_str(input);

    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(e) => {
            return Parsed {
                parse_error: Some(e.to_string()),
                ..Default::default()
            }
        }
    };

    let ast = MiniHaskellParser::build_ast_(parsed);

    let ast = match ast {
        Ok(ast) => ast,
        Err(e) => {
            return Parsed {
                build_tree_error: Some(e.to_string()),
                ..Default::default()
            }
        }
    };

    let typ_inferenec = TypeInference::new(ast.clone());

    let typ_inferenec = match typ_inferenec {
        Ok(typ_inferenec) => typ_inferenec,
        Err(e) => {
            return Parsed {
                // tree: Some(ast.to_mathjax()),
                constraints_error: Some(e.to_string()),
                ..Default::default()
            }
        }
    };

    let tree = typ_inferenec.0;
    let constraints = typ_inferenec.1;
    let constraints_str = constraints
        .iter()
        .map(|(a, b)| format!("{} = {}", a.to_mathjax(), b.to_mathjax()))
        .collect::<Vec<_>>();

    let mut all_used_vars = HashSet::<usize>::new();

    let mut maximum = constraints.clone().into_iter().fold(0, |mut max, (a, b)| {
        let res = a.all_vars().into_iter().max();
        max = cmp::max(max, res.unwrap_or(0));
        let res = b.all_vars().into_iter().max();
        max = cmp::max(max, res.unwrap_or(0));
        max
    }) + 1;

    let mut new_constraints = Vec::<RuleExpr>::new();

    for (a, b) in constraints {
        match (a, b) {
            (TypeExpr::Var(x), b) => {
                new_constraints.push(RuleExpr {
                    var: x.clone(),
                    rhs: Box::new(b.clone()),
                });
            }
            (a, TypeExpr::Var(x)) => {
                new_constraints.push(RuleExpr {
                    var: x.clone(),
                    rhs: Box::new(a.clone()),
                });
            }
            (a, b) => {
                new_constraints.push(RuleExpr {
                    var: maximum,
                    rhs: Box::new(a.clone()),
                });
                new_constraints.push(RuleExpr {
                    var: maximum,
                    rhs: Box::new(b.clone()),
                });
                maximum += 1;
            }
        }
    }

    let debug_string = new_constraints
        .iter()
        .map(|a| format!("{} ", a))
        .collect::<Vec<_>>();
    println!("{}", debug_string.join("\n"));
    // panic!();

    let solution = solve_constraints(new_constraints, 0);

    Parsed {
        tree: Some(tree.to_mathjax()),
        constraints: Some(constraints_str),
        solution: Some(solution.into()),
        ..Default::default()
    }
}
