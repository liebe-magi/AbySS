use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{AssignmentOp, ConditionalAssignment, LineInfo, OracleBranch, Type, AST};

#[derive(Parser)]
#[grammar = "abyss.pest"] // 文法ファイルを指定
pub struct AbyssParser;

pub fn parse(input: &str) -> Result<Pair<Rule>, Error<Rule>> {
    match AbyssParser::parse(Rule::statements, input) {
        Ok(mut pairs) => Ok(pairs.next().unwrap()),
        Err(e) => Err(e),
    }
}

pub fn build_ast(pair: Pair<Rule>) -> Result<AST, Error<Rule>> {
    let line_info = Some(LineInfo::from_span(&pair.as_span())); // 行・列情報を取得

    match pair.as_rule() {
        Rule::statement => build_ast(pair.into_inner().next().unwrap()),
        Rule::expression => build_ast(pair.into_inner().next().unwrap()),
        Rule::or_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap())?;
            if let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                match operator_pair.as_str() {
                    "||" => Ok(AST::LogicalOr(Box::new(left), Box::new(right), line_info)),
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
            let left = build_ast(inner.next().unwrap())?;
            if let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                match operator_pair.as_str() {
                    "&&" => Ok(AST::LogicalAnd(Box::new(left), Box::new(right), line_info)),
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

            let expr = build_ast(inner.next().unwrap())?;

            if exist_not_op {
                Ok(AST::LogicalNot(Box::new(expr), line_info))
            } else {
                Ok(expr) // `not_op` が存在しない場合は単純な`comp_expr`ノードを返す
            }
        }
        Rule::comp_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap())?;
            if let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                match operator_pair.as_str() {
                    "==" => Ok(AST::Equal(Box::new(left), Box::new(right), line_info)),
                    "!=" => Ok(AST::NotEqual(Box::new(left), Box::new(right), line_info)),
                    "<" => Ok(AST::LessThan(Box::new(left), Box::new(right), line_info)),
                    "<=" => Ok(AST::LessThanOrEqual(
                        Box::new(left),
                        Box::new(right),
                        line_info,
                    )),
                    ">" => Ok(AST::GreaterThan(Box::new(left), Box::new(right), line_info)),
                    ">=" => Ok(AST::GreaterThanOrEqual(
                        Box::new(left),
                        Box::new(right),
                        line_info,
                    )),
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
            let mut ast = build_ast(inner.next().unwrap())?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                ast = match operator_pair.as_str() {
                    "+" => AST::Add(Box::new(ast), Box::new(right), line_info.clone()),
                    "-" => AST::Subtract(Box::new(ast), Box::new(right), line_info.clone()),
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
            let mut ast = build_ast(inner.next().unwrap())?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                ast = match operator_pair.as_str() {
                    "*" => AST::Multiply(Box::new(ast), Box::new(right), line_info.clone()),
                    "/" => AST::Divide(Box::new(ast), Box::new(right), line_info.clone()),
                    "%" => AST::Modulo(Box::new(ast), Box::new(right), line_info.clone()),
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
            let mut ast = build_ast(inner.next().unwrap())?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                ast = match operator_pair.as_str() {
                    "^" => AST::PowArcana(Box::new(ast), Box::new(right), line_info.clone()),
                    "**" => AST::PowAether(Box::new(ast), Box::new(right), line_info.clone()),
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
        Rule::factor => build_ast(pair.into_inner().next().unwrap()),
        Rule::omen => {
            let value = pair.as_str();
            match value {
                "boon" => Ok(AST::Omen(true, line_info)),
                "hex" => Ok(AST::Omen(false, line_info)),
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
            Ok(AST::Arcana(value, line_info))
        }
        Rule::aether => {
            let value = pair.as_str().parse().unwrap();
            Ok(AST::Aether(value, line_info))
        }
        Rule::rune => {
            let value = pair.as_str().trim_matches('"').to_string();
            Ok(AST::Rune(value, line_info))
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

            let value = build_ast(inner.next().unwrap())?;

            Ok(AST::VarAssign {
                name: var_name,
                value: Box::new(value),
                var_type,
                is_morph,
                line_info,
            })
        }
        Rule::assignment => {
            let span = pair.as_span();
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
                    span,
                ))?,
            };
            let value = build_ast(inner.next().unwrap())?;

            Ok(AST::Assignment {
                name: var_name,
                value: Box::new(value),
                op,
                line_info,
            })
        }
        Rule::identifier => {
            let var_name = pair.as_str().to_string();
            Ok(AST::Var(var_name, line_info))
        }
        Rule::unveil => {
            let inner = pair.into_inner();
            let args: Result<Vec<AST>, Error<Rule>> = inner.map(|p| build_ast(p)).collect();
            Ok(AST::Unveil(args?, line_info))
        }
        Rule::trans_expr => {
            let span = pair.as_span(); // `pair`の`span`を取得して保持しておく
            let mut inner = pair.into_inner();
            let expr = build_ast(inner.next().unwrap())?;
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
            Ok(AST::Trans(Box::new(expr), target_type, line_info))
        }
        Rule::reveal => {
            let mut inner = pair.into_inner();
            let expression = build_ast(inner.next().unwrap())?;
            Ok(AST::Reveal(Box::new(expression), line_info))
        }
        Rule::oracle_expr => {
            let mut inner = pair.into_inner();
            let mut conditionals = Vec::new();
            let mut branches = Vec::new();
            let mut is_match = false;

            // 条件部分があるかどうかチェック
            if let Some(conditional_or_branches) = inner.peek() {
                if conditional_or_branches.as_rule() == Rule::oracle_conditional {
                    let mut condition_pair = inner.next().unwrap().into_inner();
                    let conditions = condition_pair.next().unwrap().into_inner();
                    for (idx, condition) in conditions.enumerate() {
                        if condition.as_rule() == Rule::conditional_assignment {
                            let mut inner_pairs = condition.into_inner();
                            let identifier = inner_pairs.next().unwrap().as_str().to_string();
                            let expression = build_ast(inner_pairs.next().unwrap())?;
                            conditionals.push(ConditionalAssignment {
                                variable: identifier,
                                expression: Box::new(expression),
                                line_info: line_info.clone(),
                            });
                        } else {
                            is_match = true;
                            let mut inner_pairs = condition.into_inner();
                            let expression = build_ast(inner_pairs.next().unwrap())?;
                            conditionals.push(ConditionalAssignment {
                                variable: format!("__match_{}", idx),
                                expression: Box::new(expression),
                                line_info: line_info.clone(),
                            });
                        }
                    }
                }
            }
            // ブランチ部分の解析
            for branch_pair in inner {
                let branch_span = branch_pair.as_span();
                let mut branch_inner = branch_pair.into_inner();

                if branch_inner.peek().unwrap().as_span().as_str() == "_" {
                    // default branch
                    branch_inner.next(); // branch_innerを読み飛ばす
                    let body_ast = if let Some(body) = branch_inner.next() {
                        build_ast(body)?
                    } else {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "Branch body is missing".to_string(),
                            },
                            branch_span,
                        ));
                    };
                    branches.push(OracleBranch {
                        pattern: Vec::new(),
                        body: Box::new(body_ast),
                        line_info: Some(LineInfo::from_span(&branch_span)),
                    });
                } else {
                    let rule = branch_inner
                        .peek()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_rule();
                    if rule == Rule::pattern_elements {
                        let mut expr_pairs = branch_inner.next().unwrap().into_inner();
                        let exprs = expr_pairs.next().unwrap().into_inner();
                        let mut pats = vec![];
                        for expr in exprs {
                            let pat_ast = build_ast(expr)?;
                            pats.push(pat_ast);
                        }
                        let body_ast = build_ast(branch_inner.next().unwrap())?;
                        branches.push(OracleBranch {
                            pattern: pats,
                            body: Box::new(body_ast),
                            line_info: Some(LineInfo::from_span(&branch_span)),
                        });
                    }
                }
            }
            Ok(AST::Oracle {
                is_match,
                conditionals,
                branches,
                line_info,
            })
        }
        Rule::pattern => build_ast(pair.into_inner().next().unwrap()),
        Rule::pattern_element => {
            if pair.as_span().as_str() == "_" {
                Ok(AST::OracleDontCareItem(line_info))
            } else {
                Ok(build_ast(pair.into_inner().next().unwrap())?)
            }
        }
        Rule::block => {
            let mut statements = Vec::new();
            let inner = pair.into_inner();
            for statement_pair in inner {
                let statement = build_ast(statement_pair)?;
                statements.push(statement);
            }
            Ok(AST::Block(statements, line_info))
        }
        _ => Err(Error::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Unexpected rule: {:?}", pair.as_rule()),
            },
            pair.as_span(),
        )),
    }
}
