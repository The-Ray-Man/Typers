use crate::typers::rules::{RuleExpr, RuleInfo};
use std::collections::{HashMap, HashSet, VecDeque};
use wasm_bindgen::prelude::*;

use super::mathjax::MathJax;

#[derive(Debug, Clone)]
pub struct ResultRemoveStep {
    pub id: i32,
    pub rules_before: Vec<RuleExpr>,
    pub rules_after: Vec<RuleExpr>,
    pub text: Option<String>,
    pub rules_removed: Vec<RuleExpr>,
}

impl Into::<ResultRemoveStepTS> for ResultRemoveStep {
    fn into(self) -> ResultRemoveStepTS {
        ResultRemoveStepTS {
            id: self.id,
            rules_before: self.rules_before.iter().map(|x| x.to_mathjax()).collect(),
            rules_after: self.rules_after.iter().map(|x| x.to_mathjax()).collect(),
            text: self.text,
            rules_removed: self.rules_removed.iter().map(|x| x.to_mathjax()).collect(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct ResultRemoveStepTS {
    pub id: i32,
    pub rules_before: Vec<String>,
    pub rules_after: Vec<String>,
    pub text: Option<String>,
    pub rules_removed: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResultAccumulateStep {
    pub id: i32,
    pub rules_before: Vec<RuleExpr>,
    pub rules_after: Vec<RuleExpr>,
    pub text: Option<String>,
    pub rules_added: Vec<RuleExpr>,
    pub rules_compared: (RuleExpr, RuleExpr),
}

impl Into::<ResultAccumulateStepTS> for ResultAccumulateStep {
    fn into(self) -> ResultAccumulateStepTS {
        ResultAccumulateStepTS {
            id: self.id,
            rules_before: self.rules_before.iter().map(|x| x.to_mathjax()).collect(),
            rules_after: self.rules_after.iter().map(|x| x.to_mathjax()).collect(),
            text: self.text,
            rules_added: self.rules_added.iter().map(|x| x.to_mathjax()).collect(),
            rules_compared: vec![self.rules_compared.0.to_mathjax(), self.rules_compared.1.to_mathjax()],
        }
    }
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct ResultAccumulateStepTS {
    pub id: i32,
    pub rules_before: Vec<String>,
    pub rules_after: Vec<String>,
    pub text: Option<String>,
    pub rules_added: Vec<String>,
    pub rules_compared: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResultSubstituteStep {
    pub id: i32,
    pub goal_id: usize,
    pub rules_available: Vec<RuleExpr>,
    pub rule_goal_before: RuleExpr,
    pub rule_goal_after: RuleExpr,
    pub rule_used: RuleExpr,
    pub text: Option<String>,
}

impl Into::<ResultSubstituteStepTS> for ResultSubstituteStep {
    fn into(self) -> ResultSubstituteStepTS {
        ResultSubstituteStepTS {
            id: self.id,
            goal_id: self.goal_id,
            rules_available: self.rules_available.iter().map(|x| x.to_mathjax()).collect(),
            rule_goal_before: self.rule_goal_before.to_mathjax(),
            rule_goal_after: self.rule_goal_after.to_mathjax(),
            rule_used: self.rule_used.to_mathjax(),
            text: self.text,
        }
    }
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct ResultSubstituteStepTS {
    pub id: i32,
    pub goal_id: usize,
    pub rules_available: Vec<String>,
    pub rule_goal_before: String,
    pub rule_goal_after: String,
    pub rule_used: String,
    pub text: Option<String>,
}


#[derive(Debug, Default)]
pub struct Solution {
    pub rules: Vec<RuleExpr>,
    pub variables: Vec<usize>,
    pub result_remove_steps: Vec<ResultRemoveStep>,
    pub result_accumulate_steps: Vec<ResultAccumulateStep>,
    pub result_substitute_steps: Vec<ResultSubstituteStep>,
    pub result: Option<Result<RuleExpr, String>>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct SolutionTS {
    pub rules: Vec<String>,
    pub variables: Vec<String>,
    pub result_remove_steps: Vec<ResultRemoveStepTS>,
    pub result_accumulate_steps: Vec<ResultAccumulateStepTS>,
    pub result_substitute_steps: Vec<ResultSubstituteStepTS>,
    pub result_error: Option<String>,
    pub result: Option<String>,
}

impl Into::<SolutionTS> for Solution {
    fn into(self) -> SolutionTS {
        let result_error = match self.result.clone() {
            Some(Ok(rule)) => None,
            Some(Err(e)) => Some(e.clone()),
            None => None,
        };

        let result = match self.result.clone() {
            Some(Ok(rule)) => Some(rule.clone().to_mathjax()),
            Some(Err(e)) => None,
            None => None,
        };

        SolutionTS {
            rules: self.rules.iter().map(|x| x.to_mathjax()).collect(),
            variables: self.variables.iter().map(|x| format!("t_{{{}}}", x)).collect(),
            result_remove_steps: self.result_remove_steps.iter().map(|x| (*x).clone().into()).collect(),
            result_accumulate_steps: self.result_accumulate_steps.iter().map(|x| (*x).clone().into()).collect(),
            result_substitute_steps: self.result_substitute_steps.iter().map(|x| (*x).clone().into()).collect(),
            result_error,
            result,
        }
    }
}



pub fn solve_constraints(mut rules: Vec<RuleExpr>, goal_var: usize) -> Solution {
    let mut solution = Solution::default();

    let all_rules = rules.clone();
    solution.rules = all_rules;
    let variables = variables(rules.clone());
    solution.variables = variables.clone();

    let mut accumulate_steps = Vec::<ResultAccumulateStep>::new();
    let mut remove_steps = Vec::<ResultRemoveStep>::new();
    let mut counter = 0;

    println!("1");
    loop {
        let result = accumulate_constraints(&mut rules, counter);
        match result {
            Ok(Some(step)) => {
                solution.result_accumulate_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                solution.result = Some(Err(e));
                return solution;
            }
        }

        let remove = remove_simple_rules(&mut rules, counter);

        match remove {
            Ok(Some(step)) => {
                solution.result_remove_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                solution.result = Some(Err(e));
                return solution;
            }
        }
    }

    println!("cycle");
    let res_cycle = check_cycles(&rules);

    match res_cycle {
        Ok(_) => {
            println!("No cycle found in constraints ... \n");
        }
        Err(e) => {
            solution.result = Some(Err(e));
            return solution;
        }
    }

    let mut substitute_steps = Vec::<ResultSubstituteStep>::new();
    println!("substitute");
    let mut goal_rule = find_goal_rule(&mut rules, goal_var);

    let mut goal_rule = match goal_rule {
        Ok(rule) => rule,
        Err(e) => {
            solution.result = Some(Err(e));
            return solution;
        }
    };

    loop {
        let result = substitute_constraints(&mut rules,&mut goal_rule, goal_var,  counter);
        match result {
            Ok(Some(step)) => {
                solution.result_substitute_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                solution.result = Some(Err(e));
                return solution;
            }
        }
    }


    solution.result = Some(Ok(goal_rule));
    solution
}

/// Checks if the rules contain any cycle, assumes that all left hand sides are unique, uses topological sorting
fn check_cycles(rules: &Vec<RuleExpr>) -> Result<(), String> {
    // Build graph
    let mut edge_list: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    // Build edge_list and in_degree
    for rule in rules {
        let rhs = rule.all_vars_rhs();
        in_degree.entry(rule.var).or_insert(0);
        for v in rhs.iter() {
            in_degree.entry(*v).and_modify(|x| *x += 1).or_insert(1);
        }
        edge_list.insert(rule.var, rule.all_vars_rhs());
    }

    // Enqueue all zero in_degree nodes
    let mut queue = VecDeque::new();
    for (key, val) in in_degree.iter() {
        if *val == 0 {
            queue.push_back(*key);
        }
    }

    // BFS traversal
    let mut num_visited = 0;
    while let Some(u) = queue.pop_front() {
        num_visited += 1;

        // adjust in_degree of next nodes
        if let Some(edges) = edge_list.get(&u) {
            for v in edges {
                let degree = in_degree.get_mut(v).unwrap();
                *degree -= 1;
                // if in_degree is 0 we need to enqueue the node
                if *degree == 0 {
                    queue.push_back(*v);
                }
            }
        }
    }
    if num_visited != in_degree.len() {
        Err("detected cycle in constraints, cannot proceed ...".to_string())
    } else {
        Ok(())
    }
}



fn variables(rules: Vec<RuleExpr>) -> Vec<usize> {
    let all_vars: HashSet<usize> = rules
        .all_vars_lhs()
        .union(&rules.all_vars_rhs())
        .map(|x| *x)
        .collect();
    let mut all_vars_vec: Vec<usize> = all_vars.clone().into_iter().collect();
    all_vars_vec.sort();
    all_vars_vec
}

/// Remove all rules of the form `tX = tY` by replacing `X` with `Y` in all rules (where `X` < `Y`)
/// Assumes that the goal variable has the lowest ID, since otherwise it might replace it
fn remove_simple_rules(
    rules: &mut Vec<RuleExpr>,
    counter: i32,
) -> Result<Option<ResultRemoveStep>, String> {
    let rules_before = rules.clone();
    for i in 0..rules.len() {
        if let Some((mut from, mut to)) = rules[i].is_simple() {
            if from == to {
                let error = format!("recursive definition \\(t_{{{from}}}\\) = \\(t_{{{to}}}\\)!");
                return Err(error);
            } else if from < to {
                std::mem::swap(&mut to, &mut from);
            }
            let msg = format!("Replacing \\(t_{{{from}}}\\) with \\(t_{{{to}}}\\) in all rules");
            let rule_used = rules[i].clone();
            rules.swap_remove(i);

            for rule in rules.iter_mut() {
                rule.replace_var(from, to)
            }
            let rules_after = rules.clone();
            return Ok(Some(ResultRemoveStep {
                id: counter,
                rules_before,
                rules_after,
                text: Some(msg),
                rules_removed: vec![rule_used],
            }));
        }
    }
    return Ok(None);
}

/// Accumulate constraints by comparing rules with the same left hand side and replacing variables which are equal
fn accumulate_constraints(
    rules: &mut Vec<RuleExpr>,
    counter: i32,
) -> Result<Option<ResultAccumulateStep>, String> {
    // Compare rules greedily to get new constraints

    // Iterate over rules to find two matching ones
    let mut found_new = false;
    let rules_before = rules.clone();
    'outer: for i in 0..rules.len() {
        for j in i + 1..rules.len() {
            if !rules[i].has_same_lhs(&rules[j]) {
                continue;
            }
            // Get all new constraints by comparing the rules
            if let Ok(new_rules) = rules[i].compare_rules(&rules[j]) {
                let msg = format!(
                    "Comparing these rules\n{}\n{}\nThese new rules have been found:",
                    rules[i], rules[j]
                );
                let rule_i = rules[i].clone();
                let rule_j = rules[j].clone();
                rules.swap_remove(j);
                let mut rules_after = rules.clone();
                rules_after.append(new_rules.clone().as_mut());
                rules.append(new_rules.clone().as_mut());
                return Ok(Some(ResultAccumulateStep {
                    id: counter,
                    rules_before,
                    rules_after,
                    text: Some(msg),
                    rules_added: new_rules,
                    rules_compared: (rule_i, rule_j),
                }));
            } else {
                let msg = format!(
                    "impossible to combine these rules: \\({}\\) and \\({}\\)",
                    rules[i].to_mathjax(), rules[j].to_mathjax()
                );
                return Err(msg);
            }
        }
    }
    Ok(None)
}

/// Find rule for goal variable
fn find_goal_rule(rules: &mut Vec<RuleExpr>, goal_var: usize) -> Result<RuleExpr, String> {
    match rules.iter().find(|r| r.has_lhs(goal_var)) {
        Some(ref goal_rule) => Ok((*goal_rule).clone()),
        None => {
            let msg = format!("could not find a constraint with \\(t_{{{goal_var}}}\\) on the left hand side");
            Err(msg)
        }
    }
}

/// Substitute constraints, assumes that there is no cycle in the rules
fn substitute_constraints(
    rules: &mut Vec<RuleExpr>,
    goal_rule: &mut RuleExpr,
    goal_var: usize,
    counter: i32,
) -> Result<Option<ResultSubstituteStep>, String> {

    let old_goal_rule = goal_rule.clone();
    // Substitute constraints one by one
    let rule_goal_before = goal_rule.clone();
    let rules_before = rules.clone();
    if let Some(rule) = goal_rule.substitute_constraint(&rules) {


        let rule_goal_after = goal_rule.clone();

        return Ok(Some(ResultSubstituteStep {
            id: counter,
            goal_id: goal_var,
            rules_available: rules_before,
            rule_goal_before: rule_goal_before,
            rule_goal_after: rule_goal_after,
            rule_used: rule.clone(),
            text: None,
        }));
    }
    Ok(None)
}
