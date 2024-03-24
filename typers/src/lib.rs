pub mod typers;

use std::{cmp};

use typers::{
    parser::MiniHaskellParser, rules::RuleExpr, solver::solve_constraints, tree::TypeInference,
    utils::mathjax::MathJax,
};

use typers::rules::TypeExpr;
use typers::utils::wasm::SolutionTS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Default)]
/// Represents the parsed input in a format which can be used with web assembly.
pub struct Parsed {
    pub parse_error: Option<String>, // Exists if the input is not valid MiniHaskell.
    pub build_tree_error: Option<String>, // Exists if the AST could not be built. Should never happen. Because the Grammar and the AST building process should be equivalent.
    pub tree: Option<String>,             // The AST in a format which can be rendered by MathJax.s
    pub constraints_error: Option<String>, // Error message if the constraints could not be generated.
    pub constraints: Option<Vec<String>>, // The constraints in a format which can be rendered by MathJax.
    pub constraints_without_trivial: Option<Vec<String>>, // The constraints without the trivial constraints in a format which can be rendered by MathJax.
    pub solution: Option<SolutionTS>, // The solution in a format which can be rendered by MathJax.
}

#[wasm_bindgen]
pub fn solve(input: &str) -> Parsed {
    let mut result = Parsed::default();

    // Parse the input string
    let parsed = MiniHaskellParser::parse_str(input);
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(e) => {
            result.parse_error = Some(e);
            return result;
        }
    };

    // build the AST from the parsed input.
    let ast = MiniHaskellParser::build_ast(parsed);

    let ast = match ast {
        Ok(ast) => ast,
        Err(e) => {
            // This should never happen, because the grammar and the AST building process should be equivalent.
            result.build_tree_error = Some(e);
            return result;
        }
    };

    // Generate the type constraints from the AST.
    let typ_inference = TypeInference::infer(ast.clone());

    // Tree contains the derivation tree for the expression.
    // constraints contains all the constraints that were generated while building the tree.
    let (tree, constraints) = match typ_inference {
        Ok(typ_inference) => typ_inference,
        Err(e) => {
            result.constraints_error = Some(e.to_string());
            return result;
        }
    };

    result.tree = Some(tree.to_mathjax());

    // All the constraints found in the tree.
    result.constraints = Some(
        constraints
            .iter()
            .map(|(a, b)| format!("{} = {}", a.to_mathjax(), b.to_mathjax()))
            .collect::<Vec<_>>(),
    );

    // Remove the trivial constraints from the constraints.
    result.constraints_without_trivial = Some(
        constraints
            .iter()
            .filter(|(a, b)| a != b)
            .map(|(a, b)| format!("{} = {}", a.to_mathjax(), b.to_mathjax()))
            .collect::<Vec<_>>(),
    );

    // Because the algorithm can only solve constraints of the form `t_i = [TypeExpr]` we need to convert the constraints `(TypeExpr, TypeExpr)`.
    // If the constraints are already in the correct form, we can just use them.
    // If a constraint `(TypeExpr_1, TypeExpr_2)` is not in the correct form, we need to convert them.
    // This is done by introducing a new `t_i` and adding the constraints `t_i = TypeExpr_1` and `t_i = TypeExpr_2`.

    // Get the maximum variable used in the construction
    let mut maximum = get_max_var(constraints.clone()) + 1;

    let mut new_constraints = Vec::<RuleExpr>::new();

    for (a, b) in constraints {
        match (a, b) {
            (TypeExpr::Var(x), b) => {
                // Normal form
                new_constraints.push(RuleExpr {
                    var: x,
                    rhs: Box::new(b.clone()),
                });
            }
            (a, TypeExpr::Var(x)) => {
                // Almost normal form, we need to swap the variables.
                new_constraints.push(RuleExpr {
                    var: x,
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

    // Solve the constraints
    let solution = solve_constraints(new_constraints, 0);

    result.solution = Some(solution.into());
    result
}

fn get_max_var(constraints: Vec<(TypeExpr, TypeExpr)>) -> usize {
    constraints.into_iter().fold(0, |mut max, (a, b)| {
        let res = a.all_vars().into_iter().max();
        max = cmp::max(max, res.unwrap_or(0));
        let res = b.all_vars().into_iter().max();
        max = cmp::max(max, res.unwrap_or(0));
        max
    })
}
