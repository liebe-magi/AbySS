use crate::ast::{AssignmentOp, ConditionalAssignment, LineInfo, Type, AST};
use crate::env::{Environment, Value};
use colored::*;
use std::fmt;

#[derive(Debug)]
pub enum EvalResult {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
    Rune(String),
    Abyss,
    Revealed(Box<EvalResult>),
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedVariable(String, Option<LineInfo>),
    InvalidOperation(String, Option<LineInfo>),
    NegativeExponent(Option<LineInfo>),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(var, _) => write!(f, "Variable {} is not defined!", var),
            EvalError::InvalidOperation(op, _) => write!(f, "Invalid operation: {}", op),
            EvalError::NegativeExponent(_) => {
                write!(f, "PowArcana operation requires a non-negative exponent!")
            }
        }
    }
}
impl std::error::Error for EvalError {}

pub fn display_error_with_source(script: &str, line_info: Option<LineInfo>, error_message: &str) {
    if let Some(info) = line_info {
        let lines: Vec<&str> = script.lines().collect();
        if let Some(source_line) = lines.get(info.line - 1) {
            // 行番号は1から始まるため -1
            eprintln!(
                "{}",
                format!(
                    "Error at line {}, column {}: {}",
                    info.line, info.column, error_message
                )
                .red()
            );
            eprintln!("  {}", source_line.red());
            eprintln!("  {}{}", " ".repeat(info.column - 1).red(), "^".red());
        } else {
            eprintln!("{}", format!("Error: {}", error_message).red());
        }
    } else {
        eprintln!("{}", format!("Error: {}", error_message).red());
    }
}

pub fn evaluate(ast: &AST, env: &mut Environment) -> Result<EvalResult, EvalError> {
    match ast {
        AST::Statement(node, _line_info) => evaluate(node, env),
        AST::Omen(b, _line_info) => Ok(EvalResult::Omen(*b)),
        AST::Arcana(n, _line_info) => Ok(EvalResult::Arcana(*n)),
        AST::Aether(n, _line_info) => Ok(EvalResult::Aether(*n)),
        AST::Rune(s, _line_info) => Ok(EvalResult::Rune(s.clone())),
        AST::Add(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l + r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l + r)),
            (EvalResult::Rune(l), EvalResult::Rune(r)) => {
                Ok(EvalResult::Rune(format!("{}{}", l, r)))
            }
            _ => Err(EvalError::InvalidOperation(
                "Add operation requires either two Arcana, two Aether, or two Rune!".to_string(),
                line_info.clone(),
            )),
        },
        AST::Sub(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l - r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l - r)),
            _ => Err(EvalError::InvalidOperation(
                "Subtract operation requires either two Arcana or two Aether!".to_string(),
                line_info.clone(),
            )),
        },
        AST::Mul(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l * r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l * r)),
            _ => Err(EvalError::InvalidOperation(
                "Multiply operation requires either two Arcana or two Aether!".to_string(),
                line_info.clone(),
            )),
        },
        AST::Div(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l / r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l / r)),
            _ => Err(EvalError::InvalidOperation(
                "Divide operation requires either two Arcana or two Aether!".to_string(),
                line_info.clone(),
            )),
        },
        AST::Mod(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Arcana(l % r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l % r)),
            _ => Err(EvalError::InvalidOperation(
                "Modulo operation requires either two Arcana or two Aether!".to_string(),
                line_info.clone(),
            )),
        },
        AST::PowArcana(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => {
                    if r < 0 {
                        return Err(EvalError::NegativeExponent(line_info.clone()));
                    }
                    Ok(EvalResult::Arcana(l.pow(r as u32)))
                }
                _ => Err(EvalError::InvalidOperation(
                    "PowArcana operation requires two Arcana!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::PowAether(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Aether(l.powf(r))),
                _ => Err(EvalError::InvalidOperation(
                    "PowAether operation requires two Aether!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::Equal(left, right, line_info) => match (evaluate(left, env)?, evaluate(right, env)?) {
            (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l == r)),
            (EvalResult::Aether(l), EvalResult::Aether(r)) => {
                Ok(EvalResult::Omen((l - r).abs() < std::f64::EPSILON))
            }
            (EvalResult::Rune(l), EvalResult::Rune(r)) => Ok(EvalResult::Omen(l == r)),
            _ => Err(EvalError::InvalidOperation(
                "Comparison requires compatible types!".to_string(),
                line_info.clone(),
            )),
        },
        AST::NotEqual(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l != r)),
                (EvalResult::Aether(l), EvalResult::Aether(r)) => {
                    Ok(EvalResult::Omen((l - r).abs() >= std::f64::EPSILON))
                }
                (EvalResult::Rune(l), EvalResult::Rune(r)) => Ok(EvalResult::Omen(l != r)),
                _ => Err(EvalError::InvalidOperation(
                    "Comparison requires compatible types!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::LessThan(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l < r)),
                (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l < r)),
                _ => Err(EvalError::InvalidOperation(
                    "Comparison requires numeric types!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::LessThanOrEqual(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l <= r)),
                (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l <= r)),
                _ => Err(EvalError::InvalidOperation(
                    "Comparison requires numeric types!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::GreaterThan(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l > r)),
                (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l > r)),
                _ => Err(EvalError::InvalidOperation(
                    "Comparison requires numeric types!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::GreaterThanOrEqual(left, right, line_info) => {
            match (evaluate(left, env)?, evaluate(right, env)?) {
                (EvalResult::Arcana(l), EvalResult::Arcana(r)) => Ok(EvalResult::Omen(l >= r)),
                (EvalResult::Aether(l), EvalResult::Aether(r)) => Ok(EvalResult::Omen(l >= r)),
                _ => Err(EvalError::InvalidOperation(
                    "Comparison requires numeric types!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::LogicalAnd(left, right, line_info) => {
            let left_result = evaluate(left, env)?;
            let right_result = evaluate(right, env)?;
            match (left_result, right_result) {
                (EvalResult::Omen(l), EvalResult::Omen(r)) => Ok(EvalResult::Omen(l && r)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalAnd operation requires two Omen!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::LogicalOr(left, right, line_info) => {
            let left_result = evaluate(left, env)?;
            let right_result = evaluate(right, env)?;
            match (left_result, right_result) {
                (EvalResult::Omen(l), EvalResult::Omen(r)) => Ok(EvalResult::Omen(l || r)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalOr operation requires two Omen!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::LogicalNot(expr, line_info) => {
            let result = evaluate(expr, env)?;
            match result {
                EvalResult::Omen(value) => Ok(EvalResult::Omen(!value)),
                _ => Err(EvalError::InvalidOperation(
                    "LogicalNot operation requires Omen!".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::VarAssign {
            name,
            value,
            var_type,
            is_morph,
            line_info,
        } => {
            let value = match evaluate(value, env)? {
                EvalResult::Omen(b) if *var_type == Type::Omen => Value::Omen(b),
                EvalResult::Arcana(n) if *var_type == Type::Arcana => Value::Arcana(n),
                EvalResult::Aether(n) if *var_type == Type::Aether => Value::Aether(n),
                EvalResult::Rune(s) if *var_type == Type::Rune => Value::Rune(s),
                _ => {
                    return Err(EvalError::InvalidOperation(
                        "VarAssign operation requires a valid type!".to_string(),
                        line_info.clone(),
                    ))
                }
            };
            env.set_var(name.clone(), value, var_type.clone(), *is_morph);
            Ok(EvalResult::Abyss)
        }
        AST::Assignment {
            name,
            value,
            op,
            line_info,
        } => {
            let evaluated_value = evaluate(value, env)?;

            if let Some(var_info) = env.get_var(name) {
                if !var_info.is_morph {
                    return Err(EvalError::InvalidOperation(
                        format!("Cannot reassign to immutable variable {}", name),
                        line_info.clone(),
                    ));
                }

                let result = match (evaluated_value, &var_info.value, op) {
                    (EvalResult::Arcana(v), Value::Arcana(current), op) => {
                        let new_value = match op {
                            AssignmentOp::AddAssign => current + v,
                            AssignmentOp::SubAssign => current - v,
                            AssignmentOp::MulAssign => current * v,
                            AssignmentOp::DivAssign => current / v,
                            AssignmentOp::ModAssign => current % v,
                            AssignmentOp::PowArcanaAssign => {
                                if v < 0 {
                                    return Err(EvalError::NegativeExponent(line_info.clone()));
                                }
                                current.pow(v as u32)
                            }
                            AssignmentOp::Assign => v,
                            _ => {
                                return Err(EvalError::InvalidOperation(
                                    format!("Unsupported operation for variable {}", name),
                                    line_info.clone(),
                                ))
                            }
                        };
                        env.update_var(
                            name,
                            Value::Arcana(new_value),
                            Type::Arcana,
                            line_info.clone(),
                        )
                    }
                    (EvalResult::Aether(v), Value::Aether(current), op) => {
                        let new_value = match op {
                            AssignmentOp::AddAssign => current + v,
                            AssignmentOp::SubAssign => current - v,
                            AssignmentOp::MulAssign => current * v,
                            AssignmentOp::DivAssign => current / v,
                            AssignmentOp::ModAssign => current % v,
                            AssignmentOp::PowAetherAssign => current.powf(v),
                            AssignmentOp::Assign => v,
                            _ => {
                                return Err(EvalError::InvalidOperation(
                                    format!("Unsupported operation for variable {}", name),
                                    line_info.clone(),
                                ))
                            }
                        };
                        env.update_var(
                            name,
                            Value::Aether(new_value),
                            Type::Aether,
                            line_info.clone(),
                        )
                    }
                    (EvalResult::Rune(v), Value::Rune(current), op) => {
                        let new_value = match op {
                            AssignmentOp::AddAssign => format!("{}{}", current, v),
                            AssignmentOp::Assign => v,
                            _ => {
                                return Err(EvalError::InvalidOperation(
                                    format!("Unsupported operation for variable {}", name),
                                    line_info.clone(),
                                ))
                            }
                        };
                        env.update_var(name, Value::Rune(new_value), Type::Rune, line_info.clone())
                    }
                    (EvalResult::Omen(v), _, AssignmentOp::Assign) => {
                        env.update_var(name, Value::Omen(v), Type::Omen, line_info.clone())
                    }
                    _ => Err(EvalError::InvalidOperation(
                        format!(
                            "Type mismatch or unsupported operation for variable {}",
                            name
                        ),
                        line_info.clone(),
                    )),
                };

                result.map(|_| EvalResult::Abyss)
            } else {
                Err(EvalError::UndefinedVariable(
                    name.clone(),
                    line_info.clone(),
                ))
            }
        }
        AST::Var(name, line_info) => match env.get_var(name) {
            Some(var_info) => match &var_info.value {
                Value::Omen(b) => Ok(EvalResult::Omen(*b)),
                Value::Arcana(n) => Ok(EvalResult::Arcana(*n)),
                Value::Aether(n) => Ok(EvalResult::Aether(*n)),
                Value::Rune(s) => Ok(EvalResult::Rune(s.clone())),
            },
            None => Err(EvalError::UndefinedVariable(
                name.clone(),
                line_info.clone(),
            )),
        },
        AST::Unveil(args, _line_info) => {
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
                    _ => Err(EvalError::InvalidOperation(
                        "Unsupported type in unveil statement".to_string(),
                        None,
                    )),
                })
                .collect();
            let output_str = outputs?.join("");
            println!("{}", output_str);
            Ok(EvalResult::Abyss)
        }
        AST::Trans(expr, target_type, line_info) => {
            let value = evaluate(expr, env)?;
            match target_type {
                Type::Arcana => match value {
                    EvalResult::Aether(n) => Ok(EvalResult::Arcana(n as i64)),
                    EvalResult::Rune(s) => s.parse::<i64>().map(EvalResult::Arcana).map_err(|_| {
                        EvalError::InvalidOperation(
                            "Failed to convert Rune to Arcana".to_string(),
                            line_info.clone(),
                        )
                    }),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Arcana".to_string(),
                        line_info.clone(),
                    )),
                },
                Type::Aether => match value {
                    EvalResult::Arcana(n) => Ok(EvalResult::Aether(n as f64)),
                    EvalResult::Rune(s) => s.parse::<f64>().map(EvalResult::Aether).map_err(|_| {
                        EvalError::InvalidOperation(
                            "Failed to convert Rune to Aether".to_string(),
                            line_info.clone(),
                        )
                    }),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Aether".to_string(),
                        line_info.clone(),
                    )),
                },
                Type::Rune => match value {
                    EvalResult::Arcana(n) => Ok(EvalResult::Rune(n.to_string())),
                    EvalResult::Aether(n) => Ok(EvalResult::Rune(n.to_string())),
                    _ => Err(EvalError::InvalidOperation(
                        "Invalid cast to Rune".to_string(),
                        line_info.clone(),
                    )),
                },
                Type::Omen => Err(EvalError::InvalidOperation(
                    "Casting to Omen is not supported".to_string(),
                    line_info.clone(),
                )),
            }
        }
        AST::Oracle {
            is_match,
            conditionals,
            branches,
            line_info,
        } => {
            env.push_scope(); // 新しいスコープを作成

            // 条件式を評価して環境に代入するヘルパー関数
            let mut evaluate_and_set_var =
                |conditional: &ConditionalAssignment| -> Result<(), EvalError> {
                    let result = evaluate(&conditional.expression, env)?;
                    match result {
                        EvalResult::Arcana(n) => env.set_var(
                            conditional.variable.clone(),
                            Value::Arcana(n),
                            Type::Arcana,
                            false,
                        ),
                        EvalResult::Aether(n) => env.set_var(
                            conditional.variable.clone(),
                            Value::Aether(n),
                            Type::Aether,
                            false,
                        ),
                        EvalResult::Rune(ref s) => env.set_var(
                            conditional.variable.clone(),
                            Value::Rune(s.clone()),
                            Type::Rune,
                            false,
                        ),
                        EvalResult::Omen(b) => env.set_var(
                            conditional.variable.clone(),
                            Value::Omen(b),
                            Type::Omen,
                            false,
                        ),
                        _ => {
                            return Err(EvalError::InvalidOperation(
                                format!("Unsupported type in oracle conditional: {:?}", result),
                                line_info.clone(),
                            ))
                        }
                    }
                    Ok(())
                };

            // 条件式が指定されている場合、すべての条件式を評価して環境に代入
            for conditional in conditionals {
                evaluate_and_set_var(conditional)?;
            }

            // 分岐の評価処理
            for branch in branches {
                let matched = if branch.pattern.is_empty() {
                    // デフォルトブランチ
                    true
                } else if *is_match {
                    // パターンマッチング
                    let mut matched = true;
                    for (idx, pattern) in branch.pattern.iter().enumerate() {
                        // patternがOracleDontCareItemの場合は条件を満たす
                        if let AST::OracleDontCareItem(_) = pattern {
                            continue;
                        }
                        let pattern_result = evaluate(pattern, env)?;
                        let conditional_result = evaluate(&conditionals[idx].expression, env)?;

                        match (conditional_result, pattern_result) {
                            (EvalResult::Arcana(cond_n), EvalResult::Arcana(pat_n)) => {
                                if cond_n != pat_n {
                                    matched = false;
                                    break;
                                }
                            }
                            (EvalResult::Aether(cond_n), EvalResult::Aether(pat_n)) => {
                                if (cond_n - pat_n).abs() >= std::f64::EPSILON {
                                    matched = false;
                                    break;
                                }
                            }
                            (EvalResult::Rune(cond_s), EvalResult::Rune(pat_s)) => {
                                if cond_s != pat_s {
                                    matched = false;
                                    break;
                                }
                            }
                            (EvalResult::Omen(cond_b), EvalResult::Omen(pat_b)) => {
                                if cond_b != pat_b {
                                    matched = false;
                                    break;
                                }
                            }
                            _ => {
                                return Err(EvalError::InvalidOperation(
                                    "Oracle branch pattern type must match conditional type"
                                        .to_string(),
                                    line_info.clone(),
                                ))
                            }
                        }
                    }
                    matched
                } else {
                    // Omen型パターンマッチング
                    branch
                        .pattern
                        .iter()
                        .all(|pattern| matches!(evaluate(pattern, env), Ok(EvalResult::Omen(true))))
                };

                // パターンが一致したらブランチを評価して終了
                if matched {
                    let result = match evaluate(&branch.body, env) {
                        Ok(result) => match result {
                            EvalResult::Revealed(revealed) => *revealed,
                            _ => result,
                        },
                        Err(e) => return Err(e),
                    };
                    env.pop_scope(); // スコープを終了
                    return Ok(result);
                }
            }

            env.pop_scope(); // スコープを終了
            Err(EvalError::InvalidOperation(
                "No matching pattern found in oracle statement".to_string(),
                line_info.clone(),
            ))
        }
        AST::Reveal(expr, _line_info) => {
            let result = evaluate(expr, env)?;
            Ok(EvalResult::Revealed(Box::new(result)))
        }
        AST::Block(statements, _line_info) => {
            let mut last_result = EvalResult::Abyss;
            for statement in statements {
                let result = evaluate(statement, env)?;

                if let EvalResult::Revealed(revealed) = result {
                    return Ok(*revealed);
                }
                last_result = result;
            }
            Ok(last_result)
        }
        AST::OracleDontCareItem(_line_info) => {
            Ok(EvalResult::Omen(true)) // ワイルドカードは常に真
        }
        AST::Comment(_, _) => {
            Ok(EvalResult::Abyss) // コメントは何もしない
        }
    }
}
