use crate::ast::AST;
use std::collections::HashMap;

pub enum EvalResult {
    Number(i64),
    Text(String),
    Void,
}

pub enum Value {
    Number(i64),
    Text(String),
}

pub struct Environment {
    vars: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }
}

fn evaluate(ast: &AST, env: &mut Environment) -> EvalResult {
    match ast {
        AST::Number(n) => EvalResult::Number(*n),
        AST::String(s) => EvalResult::Text(s.clone()),
        AST::Add(left, right) => match (evaluate(left, env), evaluate(right, env)) {
            (EvalResult::Number(l), EvalResult::Number(r)) => EvalResult::Number(l + r),
            (EvalResult::Text(l), EvalResult::Text(r)) => EvalResult::Text(format!("{}{}", l, r)),
            _ => panic!("Add operation requires either two numbers or two strings"),
        },
        AST::Subtract(left, right) => {
            if let (EvalResult::Number(left), EvalResult::Number(right)) =
                (evaluate(left, env), evaluate(right, env))
            {
                EvalResult::Number(left - right)
            } else {
                panic!("Subtract operation requires two numbers!");
            }
        }
        AST::Multiply(left, right) => {
            if let (EvalResult::Number(left), EvalResult::Number(right)) =
                (evaluate(left, env), evaluate(right, env))
            {
                EvalResult::Number(left * right)
            } else {
                panic!("Multiply operation requires two numbers!");
            }
        }
        AST::Divide(left, right) => {
            if let (EvalResult::Number(left), EvalResult::Number(right)) =
                (evaluate(left, env), evaluate(right, env))
            {
                EvalResult::Number(left / right)
            } else {
                panic!("Divide operation requires two numbers!");
            }
        }
        AST::VarAssign(name, expr) => {
            if let EvalResult::Number(val) = evaluate(expr, env) {
                env.set_var(name.clone(), Value::Number(val));
                EvalResult::Void
            } else {
                panic!("Variable assignment requires a number!");
            }
        }
        AST::RuneVarAssign(name, expr) => {
            if let EvalResult::Text(val) = evaluate(expr, env) {
                env.set_var(name.clone(), Value::Text(val));
                EvalResult::Void
            } else {
                panic!("Variable assignment requires a string!");
            }
        }
        AST::Var(name) => match env.get_var(name) {
            Some(Value::Number(n)) => EvalResult::Number(*n),
            Some(Value::Text(s)) => EvalResult::Text(s.clone()),
            None => panic!("Variable not found: {}", name),
        },
        AST::Unveil(expr) => {
            let val = evaluate(expr, env);
            match &val {
                EvalResult::Number(n) => println!("{}", n),
                EvalResult::Text(s) => println!("{}", s),
                EvalResult::Void => (),
            }
            val
        }
    }
}

pub fn evaluate_statements(statements: &[AST], env: &mut Environment) -> EvalResult {
    let mut last_result = EvalResult::Void;
    for stmt in statements {
        println!("Evaluating: {:?}", stmt);
        last_result = evaluate(stmt, env);
    }
    last_result
}
