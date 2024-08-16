use crate::ast::AST;
use std::collections::HashMap;
use std::fmt;

pub enum EvalResult {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
    Rune(String),
    Abyss,
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedVariable(String),
    InvalidOperation(String),
    NegativeExponent,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(var) => write!(f, "Variable {} is not defined!", var),
            EvalError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            EvalError::NegativeExponent => {
                write!(f, "PowArcana operation requires a non-negative exponent!")
            }
        }
    }
}

impl std::error::Error for EvalError {}

pub enum Value {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
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

pub fn evaluate(ast: &AST, env: &mut Environment) -> Result<EvalResult, EvalError> {
    match ast {
        AST::Omen(b) => Ok(EvalResult::Omen(*b)),
        AST::Arcana(n) => Ok(EvalResult::Arcana(*n)),
        AST::Aether(n) => Ok(EvalResult::Aether(*n)),
        AST::Rune(s) => Ok(EvalResult::Rune(s.clone())),
        AST::Add(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l + r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l + r)),
            (EvalResult::Rune(l), EvalResult::Rune(r)) => {
                Ok(EvalResult::Rune(format!("{}{}", l, r)))
            }
            _ => Err(EvalError::InvalidOperation(
                "Add operation requires either two Arcana, two Aether, or two Rune!".to_string(),
            )),
        },
        AST::Subtract(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l - r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l - r)),
            _ => Err(EvalError::InvalidOperation(
                "Subtract operation requires either two Arcana or two Aether!".to_string(),
            )),
        },
        AST::Multiply(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l * r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l * r)),
            _ => Err(EvalError::InvalidOperation(
                "Multiply operation requires either two Arcana or two Aether!".to_string(),
            )),
        },
        AST::Divide(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l / r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l / r)),
            _ => Err(EvalError::InvalidOperation(
                "Divide operation requires either two Arcana or two Aether!".to_string(),
            )),
        },
        AST::PowArcana(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => {
                if r < 0 {
                    return Err(EvalError::NegativeExponent);
                }
                Ok(EvalResult::Arcana(l.pow(r as u32)))
            }
            _ => Err(EvalError::InvalidOperation(
                "PowArcana operation requires two Arcana!".to_string(),
            )),
        },
        AST::PowAether(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l.powf(r))),
            _ => Err(EvalError::InvalidOperation(
                "PowAether operation requires two Aether!".to_string(),
            )),
        },
        AST::VarAssign(name, value) => {
            let value = match evaluate(value, env)? {
                EvalResult::Omen(b) => Value::Omen(b),
                EvalResult::Arcana(n) => Value::Arcana(n),
                EvalResult::Aether(n) => Value::Aether(n),
                EvalResult::Rune(s) => Value::Rune(s),
                _ => {
                    return Err(EvalError::InvalidOperation(
                        "VarAssign operation requires either Omen, Arcana, Aether, or Rune!"
                            .to_string(),
                    ))
                }
            };
            env.set_var(name.clone(), value);
            Ok(EvalResult::Abyss)
        }
        AST::Var(name) => match env.get_var(name) {
            Some(Value::Omen(b)) => Ok(EvalResult::Omen(*b)),
            Some(Value::Arcana(n)) => Ok(EvalResult::Arcana(*n)),
            Some(Value::Aether(n)) => Ok(EvalResult::Aether(*n)),
            Some(Value::Rune(s)) => Ok(EvalResult::Rune(s.clone())),
            None => Err(EvalError::UndefinedVariable(name.clone())),
        },
        AST::Unveil(args) => {
            let outputs: Result<Vec<String>, EvalError> = args
                .iter()
                .map(|arg| evaluate(arg, env))
                .collect::<Result<Vec<EvalResult>, EvalError>>()?
                .iter()
                .map(|result| match result {
                    EvalResult::Omen(b) => Ok(b.to_string()),
                    EvalResult::Arcana(n) => Ok(n.to_string()),
                    EvalResult::Aether(n) => Ok(n.to_string()),
                    EvalResult::Rune(s) => Ok(s.replace("\\n", "\n")),
                    EvalResult::Abyss => Ok("".to_string()),
                })
                .collect();
            let output_str = outputs?.join("");
            println!("{}", output_str);
            Ok(EvalResult::Abyss)
        }
    }
}
