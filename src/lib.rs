mod utils;
mod typers;

use typers::{parser::{AstNode, MiniHaskellParser}, tree::{TreeTS, TypeInference}};

use wasm_bindgen::prelude::*;


// #[wasm_bindgen(getter_with_clone)]
// #[derive(Debug, Clone)]
// pub struct Parsed {
//     pub tree: TreeTS,
//     pub constraints: Vec<String>,
// }


// #[wasm_bindgen]
// pub fn parse_input(input: &str) -> Result<Parsed, String> {
//     let result = MiniHaskellParser::get_ast(input)?;
//     let (tree, constraints) = TypeInference::new(result)?;
//     let parsed = Parsed {
//         tree: tree.into(),
//         constraints: constraints.into_iter().map(|(a, b)| format!("{} = {}", a, b)).collect()
//     };
//     Ok(parsed)
// }


#[wasm_bindgen]
pub fn hello_world() -> i32 {
    69
}
