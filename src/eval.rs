use crate::ast::{Type, AST};
use crate::env::{Environment, Value};
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
        AST::Equal(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l == r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => {
                Ok(EvalResult::Omen((l - r).abs() < std::f64::EPSILON))
            }
            (EvalResult::Rune(l), EvalResult::Rune(r)) => Ok(EvalResult::Omen(l == r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires compatible types!".to_string(),
            )),
        },
        AST::NotEqual(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l != r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => {
                Ok(EvalResult::Omen((l - r).abs() >= std::f64::EPSILON))
            }
            (EvalResult::Rune(l), EvalResult::Rune(r)) => Ok(EvalResult::Omen(l != r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires compatible types!".to_string(),
            )),
        },
        AST::LessThan(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l < r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l < r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires numeric types!".to_string(),
            )),
        },
        AST::LessThanOrEqual(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l <= r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l <= r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires numeric types!".to_string(),
            )),
        },
        AST::GreaterThan(left, right) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l > r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l > r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires numeric types!".to_string(),
            )),
        },
        AST::GreaterThanOrEqual(left, right) => match (evaluate(left, env)?, evaluate(right, env)?)
        {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l >= r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l >= r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires numeric types!".to_string(),
            )),
        },
        AST::LogicalAnd(left, right) => {
            let left_result = evaluate(left, env)?;
            let right_result = evaluate(right, env)?;
            match (left_result, right_result) {
                (EvalResult::Omen(true), EvalResult::Omen(true)) => Ok(EvalResult::Omen(true)),
                (EvalResult::Omen(true), EvalResult::Omen(false)) => Ok(EvalResult::Omen(false)),
                (EvalResult::Omen(false), EvalResult::Omen(true)) => Ok(EvalResult::Omen(false)),
                (EvalResult::Omen(false), EvalResult::Omen(false)) => Ok(EvalResult::Omen(false)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalAnd operation requires two Omen!".to_string(),
                )),
            }
        }
        AST::LogicalOr(left, right) => {
            let left_result = evaluate(left, env)?;
            let right_result = evaluate(right, env)?;
            match (left_result, right_result) {
                (EvalResult::Omen(true), EvalResult::Omen(true)) => Ok(EvalResult::Omen(true)),
                (EvalResult::Omen(true), EvalResult::Omen(false)) => Ok(EvalResult::Omen(true)),
                (EvalResult::Omen(false), EvalResult::Omen(true)) => Ok(EvalResult::Omen(true)),
                (EvalResult::Omen(false), EvalResult::Omen(false)) => Ok(EvalResult::Omen(false)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalOr operation requires two Omen!".to_string(),
                )),
            }
        }
        AST::LogicalNot(expr) => {
            let result = evaluate(expr, env)?;
            match result {
                EvalResult::Omen(value) => Ok(EvalResult::Omen(!value)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalNot operation requires Omen!".to_string(),
                )),
            }
        }
        AST::VarAssign {
            name,
            value,
            var_type,
            is_morph,
        } => {
            let value = match evaluate(value, env)? {
                EvalResult::Omen(b) if *var_type == Type::Omen => Value::Omen(b),
                EvalResult::Arcana(n) if *var_type == Type::Arcana => Value::Arcana(n),
                EvalResult::Aether(n) if *var_type == Type::Aether => Value::Aether(n),
                EvalResult::Rune(s) if *var_type == Type::Rune => Value::Rune(s),
                EvalResult::Omen(_)
                | EvalResult::Arcana(_)
                | EvalResult::Aether(_)
                | EvalResult::Rune(_) => {
                    return Err(EvalError::InvalidOperation(format!(
                        "VarAssign operation requires {}!",
                        match *var_type {
                            Type::Omen => "Omen",
                            Type::Arcana => "Arcana",
                            Type::Aether => "Aether",
                            Type::Rune => "Rune",
                        }
                    )));
                }
                _ => {
                    return Err(EvalError::InvalidOperation(
                        "VarAssign operation requires either Omen, Arcana, Aether, or Rune!"
                            .to_string(),
                    ))
                }
            };
            env.set_var(name.clone(), value, var_type.clone(), *is_morph);
            Ok(EvalResult::Abyss)
        }
        AST::Assignment { name, value } => {
            let evaluated_value = evaluate(value, env)?;

            // 変数が存在するかチェック
            if let Some(var_info) = env.get_var(name) {
                // is_morph でない場合はエラーを返す
                if !var_info.is_morph {
                    return Err(EvalError::InvalidOperation(format!(
                        "Cannot reassign to immutable variable {}",
                        name
                    )));
                }

                // 型が一致しているか確認して変数を更新
                let result = match (evaluated_value, &var_info.var_type) {
                    (EvalResult::Omen(b), Type::Omen) => {
                        env.update_var(name, Value::Omen(b), Type::Omen)
                    }
                    (EvalResult::Arcana(n), Type::Arcana) => {
                        env.update_var(name, Value::Arcana(n), Type::Arcana)
                    }
                    (EvalResult::Aether(n), Type::Aether) => {
                        env.update_var(name, Value::Aether(n), Type::Aether)
                    }
                    (EvalResult::Rune(s), Type::Rune) => {
                        env.update_var(name, Value::Rune(s), Type::Rune)
                    }
                    _ => Err(EvalError::InvalidOperation(format!(
                        "Type mismatch: cannot assign to variable {}",
                        name
                    ))),
                };

                // update_var が成功した場合、Ok(EvalResult::Abyss) を返す
                result.map(|_| EvalResult::Abyss)
            } else {
                Err(EvalError::UndefinedVariable(name.clone()))
            }
        }
        AST::Var { name, var_type, .. } => match env.get_var(name) {
            Some(var_info) => {
                // 型が一致しているか確認
                if var_info.var_type != *var_type {
                    return Err(EvalError::InvalidOperation(format!(
                        "Type mismatch: variable {} is of type {:?}, but {:?} was expected",
                        name, var_info.var_type, var_type
                    )));
                }

                // 変数の値を評価結果として返す
                match &var_info.value {
                    Value::Omen(b) => Ok(EvalResult::Omen(*b)),
                    Value::Arcana(n) => Ok(EvalResult::Arcana(*n)),
                    Value::Aether(n) => Ok(EvalResult::Aether(*n)),
                    Value::Rune(s) => Ok(EvalResult::Rune(s.clone())),
                }
            }
            None => Err(EvalError::UndefinedVariable(name.clone())),
        },
        AST::Unveil(args) => {
            let outputs: Result<Vec<String>, EvalError> = args
                .iter()
                .map(|arg| evaluate(arg, env))
                .collect::<Result<Vec<EvalResult>, EvalError>>()?
                .iter()
                .map(|result| match result {
                    EvalResult::Omen(b) => match b {
                        true => Ok("boon".to_string()),
                        false => Ok("hex".to_string()),
                    },
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
        AST::Trans(expr, target_type) => {
            let value = evaluate(expr, env)?;
            match target_type {
                Type::Arcana => match value {
                    EvalResult::Aether(n) => Ok(EvalResult::Arcana(n as i64)),
                    EvalResult::Rune(s) => s.parse::<i64>().map(EvalResult::Arcana).map_err(|_| {
                        EvalError::InvalidOperation("Failed to convert Rune to Arcana".to_string())
                    }),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Arcana".to_string(),
                    )),
                },
                Type::Aether => match value {
                    EvalResult::Arcana(n) => Ok(EvalResult::Aether(n as f64)),
                    EvalResult::Rune(s) => s.parse::<f64>().map(EvalResult::Aether).map_err(|_| {
                        EvalError::InvalidOperation("Failed to convert Rune to Aether".to_string())
                    }),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Aether".to_string(),
                    )),
                },
                Type::Rune => match value {
                    EvalResult::Arcana(n) => Ok(EvalResult::Rune(n.to_string())),
                    EvalResult::Aether(n) => Ok(EvalResult::Rune(n.to_string())),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Rune".to_string(),
                    )),
                },
                Type::Omen => Err(EvalError::InvalidOperation(
                    "Casting to Omen is not supported".to_string(),
                )),
            }
        }
    }
}
