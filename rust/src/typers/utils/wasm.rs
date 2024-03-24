use wasm_bindgen::prelude::wasm_bindgen;

use crate::typers::{
    rules::RuleExpr,
    solver::{ResultAccumulateStep, ResultRemoveStep, ResultSubstituteStep, Solution},
};

use super::mathjax::MathJax;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
// Represents the ResultRemoveStep in a format which can be used with web assembly.
pub struct ResultRemoveStepTS {
    pub id: i32,
    pub rules_before: Vec<String>,
    pub rules_after: Vec<String>,
    pub text: Option<String>,
    pub rules_removed: Vec<String>,
}

impl From<ResultRemoveStep> for ResultRemoveStepTS {
    fn from(val: ResultRemoveStep) -> Self {
        ResultRemoveStepTS {
            id: val.id,
            rules_before: val.rules_before.iter().map(|x| x.to_mathjax()).collect(),
            rules_after: val.rules_after.iter().map(|x| x.to_mathjax()).collect(),
            text: val.text,
            rules_removed: val.rules_removed.iter().map(|x| x.to_mathjax()).collect(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
// Represents the ResultAccumulateStep in a format which can be used with web assembly.
pub struct ResultAccumulateStepTS {
    pub id: i32,
    pub rules_before: Vec<String>,
    pub rules_after: Vec<String>,
    pub text: Option<String>,
    pub rules_added: Vec<String>,
    pub rules_compared: Vec<String>,
}

impl From<ResultAccumulateStep> for ResultAccumulateStepTS {
    fn from(val: ResultAccumulateStep) -> Self {
        ResultAccumulateStepTS {
            id: val.id,
            rules_before: val.rules_before.iter().map(|x| x.to_mathjax()).collect(),
            rules_after: val.rules_after.iter().map(|x| x.to_mathjax()).collect(),
            text: val.text,
            rules_added: val.rules_added.iter().map(|x| x.to_mathjax()).collect(),
            rules_compared: vec![
                val.rules_compared.0.to_mathjax(),
                val.rules_compared.1.to_mathjax(),
            ],
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
// Represents the ResultSubstituteStep in a format which can be used with web assembly.
pub struct ResultSubstituteStepTS {
    pub id: i32,
    pub goal_id: usize,
    pub rules_available: Vec<String>,
    pub rule_goal_before: String,
    pub rule_goal_after: String,
    pub rule_used: String,
    pub text: Option<String>,
}

impl From<ResultSubstituteStep> for ResultSubstituteStepTS {
    fn from(val: ResultSubstituteStep) -> Self {
        ResultSubstituteStepTS {
            id: val.id,
            goal_id: val.goal_id,
            rules_available: val.rules_available.iter().map(|x| x.to_mathjax()).collect(),
            rule_goal_before: val.rule_goal_before.to_mathjax(),
            rule_goal_after: val.rule_goal_after.to_mathjax(),
            rule_used: val.rule_used.to_mathjax(),
            text: val.text,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
// Represents the Solution in a format which can be used with web assembly.
pub struct SolutionTS {
    pub rules: Vec<String>,
    pub variables: Vec<String>,
    pub result_remove_steps: Vec<ResultRemoveStepTS>,
    pub result_accumulate_steps: Vec<ResultAccumulateStepTS>,
    pub result_substitute_steps: Vec<ResultSubstituteStepTS>,
    pub result_error: Option<String>,
    pub result: Option<String>,
}

impl From<Solution> for SolutionTS {
    fn from(val: Solution) -> Self {
        let result_error = match val.result.clone() {
            Some(Ok(_rule)) => None,
            Some(Err(e)) => Some(e.clone()),
            None => None,
        };

        let result = match val.result.clone() {
            Some(Ok(rule)) => Some(rule.clone().to_mathjax()),
            Some(Err(_e)) => None,
            None => None,
        };

        let rules = val.rules.iter().map(|x| x.to_mathjax()).collect();
        let variables = val
            .variables
            .iter()
            .map(|x| format!("t_{{{}}}", x))
            .collect();
        let result_remove_steps = val
            .result_remove_steps
            .iter()
            .map(|x| (*x).clone().into())
            .collect();
        let result_accumulate_steps = val
            .result_accumulate_steps
            .iter()
            .map(|x| (*x).clone().into())
            .collect();
        let result_substitute_steps = val
            .result_substitute_steps
            .iter()
            .map(|x| (*x).clone().into())
            .collect();

        SolutionTS {
            rules,
            variables,
            result_remove_steps,
            result_accumulate_steps,
            result_substitute_steps,
            result_error,
            result,
        }
    }
}
