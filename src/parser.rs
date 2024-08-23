use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{AssignmentOp, ConditionalAssignment, LineInfo, Type, AST};

/// The AbyssParser struct, generated using Pest, handles the parsing of the AbySS grammar.
#[derive(Parser)]
#[grammar = "abyss.pest"]
pub struct AbyssParser;

/// Parses the input string using the AbySS grammar and returns the top-level AST node.
///
/// # Arguments
/// * `input` - A string slice containing the AbySS source code.
///
/// # Returns
/// A `Result` containing a `Pair<Rule>` on success or a `pest::error::Error` on failure.
pub fn parse(input: &str) -> Result<Pair<Rule>, Error<Rule>> {
    match AbyssParser::parse(Rule::statements, input) {
        Ok(mut pairs) => Ok(pairs.next().unwrap()),
        Err(e) => Err(e),
    }
}

/// Builds the AST from a parsed pair of the AbySS grammar.
///
/// # Arguments
/// * `pair` - A `Pair<Rule>` representing a parsed node in the AbySS grammar.
///
/// # Returns
/// A `Result` containing an `AST` node on success or a `pest::error::Error` on failure.
pub fn build_ast(pair: Pair<Rule>) -> Result<AST, Error<Rule>> {
    let line_info = Some(LineInfo::from_span(&pair.as_span()));

    match pair.as_rule() {
        Rule::statement => {
            let mut inner = pair.into_inner();
            let expression = build_ast(inner.next().unwrap())?;
            Ok(AST::Statement(Box::new(expression), line_info))
        }
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
                Ok(left)
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
                Ok(left)
            }
        }
        Rule::not_expr => {
            let mut inner = pair.into_inner();

            let exist_not_op = if inner.peek().unwrap().as_rule() == Rule::not_op {
                inner.next();
                true
            } else {
                false
            };

            let expr = build_ast(inner.next().unwrap())?;

            if exist_not_op {
                Ok(AST::LogicalNot(Box::new(expr), line_info))
            } else {
                Ok(expr)
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
                Ok(left)
            }
        }
        Rule::add_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap())?;

            while let Some(operator_pair) = inner.next() {
                let right = build_ast(inner.next().unwrap())?;
                ast = match operator_pair.as_str() {
                    "+" => AST::Add(Box::new(ast), Box::new(right), line_info.clone()),
                    "-" => AST::Sub(Box::new(ast), Box::new(right), line_info.clone()),
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
                    "*" => AST::Mul(Box::new(ast), Box::new(right), line_info.clone()),
                    "/" => AST::Div(Box::new(ast), Box::new(right), line_info.clone()),
                    "%" => AST::Mod(Box::new(ast), Box::new(right), line_info.clone()),
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
            let span = pair.as_span();
            let mut inner = pair.into_inner();

            let is_morph = if inner.peek().unwrap().as_rule() == Rule::morph {
                inner.next();
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
            let span = pair.as_span();
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
                    span,
                ))?,
            };
            Ok(AST::Trans(Box::new(expr), target_type, line_info))
        }
        Rule::reveal => {
            let mut inner = pair.into_inner();
            match inner.next() {
                Some(expr) => {
                    let expression = build_ast(expr)?;
                    Ok(AST::Reveal(Box::new(expression), line_info.clone()))
                }
                None => Ok(AST::Reveal(
                    Box::new(AST::Abyss(line_info.clone())),
                    line_info.clone(),
                )),
            }
        }
        Rule::oracle_expr => {
            let mut inner = pair.into_inner();
            let mut conditionals = Vec::new();
            let mut branches = Vec::new();
            let mut is_match = false;

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

            for branch_pair in inner {
                let branch_span = branch_pair.as_span();

                if branch_pair.as_rule() == Rule::COMMENT {
                    let comment_text = branch_pair.as_str().to_string();
                    branches.push(AST::Comment(comment_text, line_info.clone()));
                    continue;
                }

                let mut branch_inner = branch_pair.into_inner();
                if branch_inner.peek().unwrap().as_span().as_str() == "_" {
                    branch_inner.next();
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
                    branches.push(AST::OracleBranch {
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
                        branches.push(AST::OracleBranch {
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
        Rule::orbit => {
            let mut inner = pair.into_inner();
            let mut params = Vec::new();
            if inner.peek().unwrap().as_rule() == Rule::orbit_params {
                let param_pairs = inner.next().unwrap().into_inner();
                for param_pair in param_pairs {
                    let param = build_ast(param_pair)?;
                    params.push(param);
                }
            }
            Ok(AST::Orbit {
                params,
                body: Box::new(build_ast(inner.next().unwrap())?),
                line_info,
            })
        }
        Rule::orbit_param => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut range_expr = inner.next().unwrap().into_inner();
            let start = build_ast(range_expr.next().unwrap())?;
            let op = range_expr.next().unwrap().as_str();
            let end = build_ast(range_expr.next().unwrap())?;
            Ok(AST::OrbitParam {
                name,
                start: Box::new(start),
                end: Box::new(end),
                op: op.to_string(),
                line_info,
            })
        }
        Rule::orbit_flow => {
            let span = pair.as_span();
            let mut inner = pair.into_inner();
            let rule = inner.peek().unwrap().as_rule();
            let identifier = match inner.next().unwrap().into_inner().next() {
                Some(id) => Some(id.as_str().to_string()),
                None => None,
            };
            match rule {
                Rule::resume_expr => Ok(AST::Resume(identifier, line_info)),
                Rule::eject_expr => Ok(AST::Eject(identifier, line_info)),
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unexpected orbit flow".to_string(),
                    },
                    span,
                )),
            }
        }
        Rule::engrave => {
            let span = pair.as_span();
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut params = Vec::new();
            if inner.peek().unwrap().as_rule() == Rule::engrave_params {
                let param_pairs = inner.next().unwrap().into_inner();
                for param_pair in param_pairs {
                    let param = build_ast(param_pair)?;
                    params.push(param);
                }
            }
            let return_type = match inner.peek().unwrap().as_rule() {
                Rule::engrave_type => {
                    let return_type = inner.next().unwrap().as_str();
                    match return_type {
                        "arcana" => Type::Arcana,
                        "aether" => Type::Aether,
                        "rune" => Type::Rune,
                        "omen" => Type::Omen,
                        "abyss" => Type::Abyss,
                        _ => Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: format!("Unknown return type in engrave: {}", return_type),
                            },
                            span,
                        ))?,
                    }
                }
                _ => Type::Abyss,
            };
            Ok(AST::Engrave {
                name,
                params,
                return_type,
                body: Box::new(build_ast(inner.next().unwrap())?),
                line_info,
            })
        }
        Rule::engrave_param => {
            let span = pair.as_span();
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let param_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Unknown type in engrave parameter".to_string(),
                    },
                    span,
                ))?,
            };
            Ok(AST::EngraveParam {
                name,
                param_type,
                line_info,
            })
        }
        Rule::func_call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut args = Vec::new();
            if let Some(peeked) = inner.peek() {
                if peeked.as_rule() == Rule::func_args {
                    let arg_pairs = inner.next().unwrap().into_inner();
                    for arg_pair in arg_pairs {
                        let arg = build_ast(arg_pair)?;
                        args.push(arg);
                    }
                }
            }
            Ok(AST::FuncCall {
                name,
                args,
                line_info,
            })
        }
        Rule::COMMENT => {
            let comment = pair.as_str().to_string();
            Ok(AST::Comment(comment, line_info))
        }
        _ => Err(Error::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Unexpected rule: {:?}", pair.as_rule()),
            },
            pair.as_span(),
        )),
    }
}
