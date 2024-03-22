mod utils;
mod typers;

use typers::{parser::{AstNode, MiniHaskellParser}, tree::{TreeTS, TypeInference}};

use wasm_bindgen::prelude::*;


#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Parsed {
    pub tree: TreeTS,
    pub constraints: Vec<String>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct ParseResult {
    pub result : Option<Parsed>,
    pub error : Option<String>
}

#[wasm_bindgen]
pub fn parse_input(input: &str) -> ParseResult {
    let haskell_parsed = MiniHaskellParser::get_ast(input);
    let haskell_parsed = match haskell_parsed {
        Ok(parsed) => parsed,
        Err(e) => return ParseResult {
            result: None,
            error: Some(e)
        }
    };

    
    let constraints = TypeInference::new(haskell_parsed);

    let constraints = match constraints {
        Ok(constraints) => constraints,
        Err(e) => return ParseResult {
            result: None,
            error: Some(e)
        }
    };

    let tree = constraints.0;
    let constraints = constraints.1;
    let constraints = constraints.iter().map(|(a,b)| format!("{}={}", a, b)).collect::<Vec<_>>();
    
    ParseResult {
        result: Some(Parsed {
            tree: tree.into(),
            constraints
        }),
        error: None
    }
}
