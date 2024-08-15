use crate::ast::AST;
use std::collections::HashMap;

pub enum EvalResult {
    Arcana(i64),
    Rune(String),
    Abyss,
}

pub enum Value {
    Arcana(i64),
    Rune(String),
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

pub fn evaluate(ast: &AST, env: &mut Environment) -> EvalResult {
    match ast {
        AST::Arcana(n) => EvalResult::Arcana(*n),
        AST::Rune(s) => EvalResult::Rune(s.clone()),
        AST::Add(left, right) => match (evaluate(left, env), evaluate(right, env)) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => EvalResult::Arcana(l + r),
            (EvalResult::Rune(l), EvalResult::Rune(r)) => EvalResult::Rune(format!("{}{}", l, r)),
            _ => panic!("Add operation requires either two Arcana or Rune!"),
        },
        AST::Subtract(left, right) => match (evaluate(left, env), evaluate(right, env)) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => EvalResult::Arcana(l - r),
            _ => panic!("Subtract operation requires either two Arcana!"),
        },
        AST::Multiply(left, right) => match (evaluate(left, env), evaluate(right, env)) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => EvalResult::Arcana(l * r),
            _ => panic!("Multiply operation requires either two Arcana!"),
        },
        AST::Divide(left, right) => match (evaluate(left, env), evaluate(right, env)) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => EvalResult::Arcana(l / r),
            _ => panic!("Divide operation requires either two Arcana!"),
        },
        AST::VarAssign(name, value) => {
            let value = match evaluate(value, env) {
                EvalResult::Arcana(n) => Value::Arcana(n),
                EvalResult::Rune(s) => Value::Rune(s),
                _ => panic!("VarAssign operation requires either Arcana or Rune!"),
            };
            env.set_var(name.clone(), value);
            EvalResult::Abyss
        }
        AST::Var(name) => match env.get_var(name) {
            Some(Value::Arcana(n)) => EvalResult::Arcana(*n),
            Some(Value::Rune(s)) => EvalResult::Rune(s.clone()),
            None => panic!("Variable {} is not defined!", name),
        },
        AST::Unveil(args) => {
            let outputs: Vec<String> = args
                .iter()
                .map(|arg| evaluate(arg, env))
                .collect::<Vec<EvalResult>>()
                .iter()
                .map(|result| match result {
                    EvalResult::Arcana(n) => n.to_string(),
                    EvalResult::Rune(s) => s.clone(),
                    EvalResult::Abyss => "".to_string(),
                })
                .collect();
            let output_str = outputs.join("");
            println!("{}", output_str);
            EvalResult::Abyss
        }
    }
}
