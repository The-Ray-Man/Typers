mod utils;
mod typers;

use typers::{mathjax::MathJax, parser::{AstNode, MiniHaskellParser}, tree::{TreeTS, TypeInference}};

use wasm_bindgen::prelude::*;
use typers::rules::TypeExpr;
use tsify::Tsify;

// #[wasm_bindgen]


#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Default)]
pub struct Parsed {
    pub parse_error : Option<String>,
    pub build_tree_error : Option<String>,
    pub tree: Option<String>,
    pub constraints_error : Option<String>,
    pub constraints: Option<Vec<String>>,
    pub solution_error: Option<String>,
    pub solution: Option<Vec<String>>
}






#[wasm_bindgen]
pub fn parse_input(input: &str) -> Parsed {

    let parsed = MiniHaskellParser::parse_str(input);

    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(e) => return Parsed {
            parse_error: Some(e.to_string()),
            ..Default::default()
        }
    };

    let ast = MiniHaskellParser::build_ast_(parsed);

    let ast = match ast {
        Ok(ast) => ast,
        Err(e) => return Parsed {
            build_tree_error: Some(e.to_string()),
            ..Default::default()
        }
    };

    let typ_inferenec = TypeInference::new(ast.clone());

    let typ_inferenec = match typ_inferenec {
        Ok(typ_inferenec) => typ_inferenec,
        Err(e) => return Parsed {
            tree: Some(ast.to_mathjax()),
            constraints_error: Some(e.to_string()),
            ..Default::default()
        }
    };

    let tree = typ_inferenec.0;
    let constraints = typ_inferenec.1;
    let constraints_str = constraints.iter().map(|(a,b)| format!("{} = {}", a.to_mathjax(), b.to_mathjax())).collect::<Vec<_>>();

    Parsed {
        tree: Some(tree.to_mathjax()),
        constraints: Some(constraints_str),
        ..Default::default()
    }

}
