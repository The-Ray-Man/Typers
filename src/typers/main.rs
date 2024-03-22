use std::{collections::HashMap, io::Read};

// use constraint_parser::TypeConstraintParser;
use parser::{AstNode, MiniHaskellParser};
use rules::{RuleExpr, TypeExpr};
use solver::solve_constraints;
mod macros;
// mod constraint_parser;
mod rules;
mod solver;
mod parser;
mod tree;

use clap::Parser;
use tree::Tree;

use crate::tree::TypeInference;
use std::fs;
use typst::{foundations::Dict, Library};


const ERROR: &str = "\x1B[31;1m[ERROR]\x1B[0m"; // ANSI sequence for "[ERROR]" in red and bold

#[derive(Parser)]
#[command(
    version,
    about = "A type constraint solver for mini-haskell types.\n\nExpected format:\nt0 = (t1, t2) -> t3\nt1 = t4 -> Bool\nt3 = Int"
)]
struct Cli {
    expression: String,
    // /// constraint file, if no file is provided the program will read from stdin
    // file: Option<String>,
    // #[arg(short, long, default_value = "0", name = "goal variable")]
    // // the ID of the variable for which should be solved, by default the program tries to solve for t0
    // goal_var: usize,
}




fn main() {
    let cli = Cli::parse();
    let expression = cli.expression.as_str();

    let timer = std::time::Instant::now();
    let mut inputs = Dict::new();



    let res = MiniHaskellParser::get_ast(expression);

    let ast = match res {
        Ok(ast) => {
            println!("success");
            ast
        }
        Err(e) => {
            let error = e.split("\n").collect::<Vec<&str>>();
            for line in error {
                eprintln!("{}", line);
            }
            std::process::exit(1);
        }
    };

    let res: Result<(Tree, Vec<(TypeExpr, TypeExpr)>), String> = TypeInference::new(ast);
    let (tree, constraints) = match res {
        Ok(tree) => {
            tree
        }
        Err(e) => {
            eprintln!("{ERROR}: {e}");
            std::process::exit(1);
        }
    };

    let new_constraints = constraints.iter().map(|(a, b)| {
        let n = a.compare_types(b);
        match n {
            Ok(res) => {
                res
            }
            Err(e) => {
                eprintln!("{ERROR}: {} is incompatible with {}", a, b);
                std::process::exit(1);
            }
        }
    }).flatten().collect::<Vec<RuleExpr>>();
    print!("{}", tree);
    solve_constraints(new_constraints, 0);

    // match res {
    //     Ok(tokens) => {
    //         println!("success");
    //         for token in tokens {
    //             println!("{}", token);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("[ERROR]: {:?}", e);
    //         std::process::exit(1);
    //     }
    // }

    // let (content, name) = if let Some(path) = cli.file {
    //     (read_file(&path), path)
    // } else {
    //     (read_stdin(), "stdin".to_owned())
    // };
    // let rules = match TypeConstraintParser::get_constraints(&content, &name) {
    //     Ok(x) => x,
    //     Err(e) => {
    //         eprintln!("{e}");
    //         std::process::exit(1);
    //     }
    // };
    // solve_constraints(rules, cli.goal_var);
}

/// Read everything from standard input
fn read_stdin() -> String {
    let mut sin = std::io::stdin().lock();
    let mut content = String::new();
    if let Err(e) = sin.read_to_string(&mut content) {
        eprintln!("{ERROR}: stdin contains invalid data: {e}");
        std::process::exit(1);
    }
    content
}

/// Read file, if it exists and contains valid utf-8 encoded data
fn read_file(path: &String) -> String {
    if let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(path) {
        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            eprintln!("{ERROR}: file {path} contains invalid data: {e}");
            std::process::exit(1);
        }
        content
    } else {
        eprintln!("{ERROR}: could not open file {path}");
        std::process::exit(1);
    }
}
