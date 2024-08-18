use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{AssignmentOp, Type, AST};
use crate::env::{SymbolInfo, SymbolTable};

#[derive(Parser)]
#[grammar = "abyss.pest"] // 文法ファイルを指定
pub struct AbyssParser;

pub fn parse(input: &str) -> Result<Pair<Rule>, Error<Rule>> {
    match AbyssParser::parse(Rule::statements, input) {
        Ok(mut pairs) => Ok(pairs.next().unwrap()),
        Err(e) => Err(e),
    }
}

pub fn build_ast(pair: Pair<Rule>, symbol_table: &mut SymbolTable) -> Result<AST, Error<Rule>> {
    match pair.as_rule() {
        Rule::statement => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::expression => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::or_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table)?;
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                match operator.as_str() {
                    "||" => Ok(AST::LogicalOr(Box::new(left), Box::new(right))),
                    _ => Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Unexpected logical operator".to_string(),
                        },
                        operator_pair.as_span(),
                    )),
                }
            } else {
                Ok(left) // 論理演算子がない場合は単純な`and_expr`ノードを返す
            }
        }
        Rule::and_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table)?;
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                match operator.as_str() {
                    "&&" => Ok(AST::LogicalAnd(Box::new(left), Box::new(right))),
                    _ => Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Unexpected logical operator".to_string(),
                        },
                        operator_pair.as_span(),
                    )),
                }
            } else {
                Ok(left) // 論理演算子がない場合は単純な`not_expr`ノードを返す
            }
        }
        Rule::not_expr => {
            let mut inner = pair.into_inner();

            let exist_not_op = if inner.peek().unwrap().as_rule() == Rule::not_op {
                inner.next(); // `not_op` を読み飛ばす
                true
            } else {
                false
            };

            let expr = build_ast(inner.next().unwrap(), symbol_table)?;

            if exist_not_op {
                Ok(AST::LogicalNot(Box::new(expr)))
            } else {
                Ok(expr) // `not_op` が存在しない場合は単純な`comp_expr`ノードを返す
            }
        }
        Rule::comp_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table)?;
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                match operator.as_str() {
                    "==" => Ok(AST::Equal(Box::new(left), Box::new(right))),
                    "!=" => Ok(AST::NotEqual(Box::new(left), Box::new(right))),
                    "<" => Ok(AST::LessThan(Box::new(left), Box::new(right))),
                    "<=" => Ok(AST::LessThanOrEqual(Box::new(left), Box::new(right))),
                    ">" => Ok(AST::GreaterThan(Box::new(left), Box::new(right))),
                    ">=" => Ok(AST::GreaterThanOrEqual(Box::new(left), Box::new(right))),
                    _ => Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Unexpected comparison operator".to_string(),
                        },
                        operator_pair.as_span(),
                    )),
                }
            } else {
                Ok(left) // 比較演算子がない場合は単純な`power`ノードを返す
            }
        }
        Rule::add_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table)?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                ast = match operator_pair.as_str() {
                    "+" => AST::Add(Box::new(ast), Box::new(right)),
                    "-" => AST::Subtract(Box::new(ast), Box::new(right)),
                    _ => {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Unexpected addition operator".to_string(),
                            },
                            operator_pair.as_span(),
                        ));
                    }
                };
            }
            Ok(ast)
        }
        Rule::mul_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table)?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                ast = match operator_pair.as_str() {
                    "*" => AST::Multiply(Box::new(ast), Box::new(right)),
                    "/" => AST::Divide(Box::new(ast), Box::new(right)),
                    "%" => AST::Modulo(Box::new(ast), Box::new(right)),
                    _ => {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Unexpected multiplication operator".to_string(),
                            },
                            operator_pair.as_span(),
                        ));
                    }
                };
            }
            Ok(ast)
        }
        Rule::pow_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table)?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap(), symbol_table)?;
                ast = match operator_pair.as_str() {
                    "^" => AST::PowArcana(Box::new(ast), Box::new(right)),
                    "**" => AST::PowAether(Box::new(ast), Box::new(right)),
                    _ => {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Unexpected power operator".to_string(),
                            },
                            operator_pair.as_span(),
                        ));
                    }
                };
            }
            Ok(ast)
        }
        Rule::factor => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::omen => {
            let value = pair.as_str();
            match value {
                "boon" => Ok(AST::Omen(true)),
                "hex" => Ok(AST::Omen(false)),
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unknown omen value".to_string(),
                    },
                    pair.as_span(),
                )),
            }
        }
        Rule::arcana => {
            let value = pair.as_str().parse().unwrap();
            Ok(AST::Arcana(value))
        }
        Rule::aether => {
            let value = pair.as_str().parse().unwrap();
            Ok(AST::Aether(value))
        }
        Rule::rune => {
            let value = pair.as_str().trim_matches('"').to_string();
            Ok(AST::Rune(value))
        }
        Rule::forge_var => {
            let span = pair.as_span(); // `pair`の`span`を取得して保持しておく
            let mut inner = pair.into_inner();

            let is_morph = if inner.peek().unwrap().as_rule() == Rule::morph {
                inner.next(); // morphを読み飛ばす
                true
            } else {
                false
            };

            let var_name = inner.next().unwrap().as_str().to_string();
            let var_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unknown type in forge variable".to_string(),
                    },
                    span,
                ))?,
            };

            let value = build_ast(inner.next().unwrap(), symbol_table)?;

            symbol_table.insert(
                var_name.clone(),
                SymbolInfo {
                    var_type: var_type.clone(),
                    is_morph,
                },
            );

            Ok(AST::VarAssign {
                name: var_name,
                value: Box::new(value),
                var_type,
                is_morph,
            })
        }
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let var_name = inner.next().unwrap().as_str().to_string();
            let op = match inner.next().unwrap().as_str() {
                "=" => AssignmentOp::Assign,
                "+=" => AssignmentOp::AddAssign,
                "-=" => AssignmentOp::SubAssign,
                "*=" => AssignmentOp::MulAssign,
                "/=" => AssignmentOp::DivAssign,
                "%=" => AssignmentOp::ModAssign,
                "^=" => AssignmentOp::PowArcanaAssign,
                "**=" => AssignmentOp::PowAetherAssign,
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unexpected assignment operator".to_string(),
                    },
                    inner.next().unwrap().as_span(),
                ))?,
            };
            let value = build_ast(inner.next().unwrap(), symbol_table)?;

            Ok(AST::Assignment {
                name: var_name,
                value: Box::new(value),
                op,
            })
        }
        Rule::identifier => {
            let var_name = pair.as_str().to_string();
            if let Some(symbol_info) = symbol_table.get(&var_name) {
                Ok(AST::Var {
                    name: var_name,
                    var_type: symbol_info.var_type.clone(),
                    is_morph: symbol_info.is_morph,
                })
            } else {
                Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Variable `{}` is not defined", var_name),
                    },
                    pair.as_span(),
                ))
            }
        }
        Rule::unveil => {
            let inner = pair.into_inner();
            let args: Result<Vec<AST>, Error<Rule>> =
                inner.map(|p| build_ast(p, symbol_table)).collect();
            Ok(AST::Unveil(args?))
        }
        Rule::trans_expr => {
            let span = pair.as_span(); // `pair`の`span`を取得して保持しておく
            let mut inner = pair.into_inner();
            let expr = build_ast(inner.next().unwrap(), symbol_table)?;
            let target_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unknown type in trans expression".to_string(),
                    },
                    span, // ここで保持していた`span`を使う
                ))?,
            };
            Ok(AST::Trans(Box::new(expr), target_type))
        }
        _ => Err(Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Unexpected rule".to_string(),
            },
            pair.as_span(),
        )),
    }
}
