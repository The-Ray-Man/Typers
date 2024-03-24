use crate::typers::rules::{RuleExpr, RuleInfo};
use std::collections::{HashMap, HashSet, VecDeque};


use super::utils::mathjax::MathJax;

#[derive(Debug, Clone)]
// Represents the information of a remove step
pub struct ResultRemoveStep {
    pub id: i32,
    pub rules_before: Vec<RuleExpr>,
    pub rules_after: Vec<RuleExpr>,
    pub text: Option<String>,
    pub rules_removed: Vec<RuleExpr>,
}

#[derive(Debug, Clone)]
// Represents the information of an accumulate step
pub struct ResultAccumulateStep {
    pub id: i32,
    pub rules_before: Vec<RuleExpr>,
    pub rules_after: Vec<RuleExpr>,
    pub text: Option<String>,
    pub rules_added: Vec<RuleExpr>,
    pub rules_compared: (RuleExpr, RuleExpr),
}

#[derive(Debug, Clone)]
// Represents the information of a substitute step
pub struct ResultSubstituteStep {
    pub id: i32,
    pub goal_id: usize,
    pub rules_available: Vec<RuleExpr>,
    pub rule_goal_before: RuleExpr,
    pub rule_goal_after: RuleExpr,
    pub rule_used: RuleExpr,
    pub text: Option<String>,
}

#[derive(Debug, Default)]
// Represents the solution of the constraint solving process
pub struct Solution {
    pub rules: Vec<RuleExpr>,
    pub variables: Vec<usize>,
    pub result_remove_steps: Vec<ResultRemoveStep>,
    pub result_accumulate_steps: Vec<ResultAccumulateStep>,
    pub result_substitute_steps: Vec<ResultSubstituteStep>,
    pub result: Option<Result<RuleExpr, String>>,
}

pub fn solve_constraints(mut rules: Vec<RuleExpr>, goal_var: usize) -> Solution {
    // Initialize solution
    let mut solution = Solution::default();
    solution.rules = rules.clone();
    solution.variables = variables(rules.clone());

    // Set up Vectors for the individual steps
    let _accumulate_steps = Vec::<ResultAccumulateStep>::new();
    let _remove_steps = Vec::<ResultRemoveStep>::new();
    let _substitute_steps = Vec::<ResultSubstituteStep>::new();

    // This counter is used to keep track of the order of the steps.
    // This is necessary, because they are in different vectors and we need to know the order of the steps.
    let mut counter = 0;

    loop {
        // Do a accumulate step
        let result = accumulate_constraints(&mut rules, counter);
        match result {
            Ok(Some(step)) => {
                solution.result_accumulate_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                // if nothing can be accumulated, we cannot simplify the rules any further.
                break;
            }
            Err(e) => {
                solution.result = Some(Err(e));
                return solution;
            }
        }

        let remove = remove_simple_rule(&mut rules, counter);

        match remove {
            Ok(Some(step)) => {
                solution.result_remove_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                // if nothing can be removed, we cannot simplify the rules any further.
                continue;
            }
            Err(e) => {
                solution.result = Some(Err(e));
                return solution;
            }
        }
    }

    // Check if there are infinite types
    let res_cycle = check_cycles(&rules);

    if let Err(e) = res_cycle {
        solution.result = Some(Err(e));
        return solution;
    }

    // Now we have simplified the rules as much as possible, we can substitute the constraints into the goal rule
    // Find the goal rule
    let goal_rule = find_goal_rule(&mut rules, goal_var);

    let mut goal_rule = match goal_rule {
        Ok(rule) => rule,
        Err(e) => {
            solution.result = Some(Err(e));
            return solution;
        }
    };

    // Substitute constraints until we cannot do it anymore
    loop {
        let result = substitute_constraints(&mut rules, &mut goal_rule, goal_var, counter);
        match result {
            Ok(Some(step)) => {
                solution.result_substitute_steps.push(step);
                counter += 1;
            }
            Ok(None) => {
                // if nothing can be substituted, we are done
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
        .copied()
        .collect();
    let mut all_vars_vec: Vec<usize> = all_vars.clone().into_iter().collect();
    all_vars_vec.sort();
    all_vars_vec
}

/// Remove the first rule of the form `tX = tY` by replacing `X` with `Y` in all rules (where `X` < `Y`)
/// Assumes that the goal variable has the lowest ID, since otherwise it might replace it
fn remove_simple_rule(
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
    Ok(None)
}

/// Accumulate constraints by comparing rules with the same left hand side and replacing variables which are equal
fn accumulate_constraints(
    rules: &mut Vec<RuleExpr>,
    counter: i32,
) -> Result<Option<ResultAccumulateStep>, String> {
    // Compare rules greedily to get new constraints

    // Iterate over rules to find two matching ones
    let _found_new = false;
    let rules_before = rules.clone();
    for i in 0..rules.len() {
        for j in i + 1..rules.len() {
            if !rules[i].has_same_lhs(&rules[j]) {
                continue;
            }
            // Get all new constraints by comparing the rules
            if let Ok(new_rules) = rules[i].compare_rules(&rules[j]) {
                let msg = format!("Comparing these rules\n{}\n{}", rules[i], rules[j]);
                // save the compared rules for the step
                let rule_i = rules[i].clone();
                let rule_j = rules[j].clone();

                rules.swap_remove(j);

                // save the rules for the step
                let mut rules_after = rules.clone();
                rules_after.append(new_rules.clone().as_mut());

                // Add the new rules to our working set
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
                // Two rules with the same lhs was found, but they cannot be combined. Therefore the constraints are inconsistent.
                let msg = format!(
                    "impossible to combine these rules: \\({}\\) and \\({}\\)",
                    rules[i].to_mathjax(),
                    rules[j].to_mathjax()
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
        Some(goal_rule) => Ok(goal_rule.clone()),
        None => {
            let msg = format!(
                "could not find a constraint with \\(t_{{{goal_var}}}\\) on the left hand side"
            );
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
    // Save the goal rule before substitution
    let rule_goal_before = goal_rule.clone();
    let rules_before = rules.clone();

    // Substitute constraints one by one
    if let Some(rule) = goal_rule.substitute_constraint(rules) {
        // Save the goal rule after substitution
        let rule_goal_after = goal_rule.clone();

        return Ok(Some(ResultSubstituteStep {
            id: counter,
            goal_id: goal_var,
            rules_available: rules_before,
            rule_goal_before,
            rule_goal_after,
            rule_used: rule.clone(),
            text: None,
        }));
    }
    Ok(None)
}
