use pest::error::Error;
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

pub fn build_ast(pair: Pair<Rule>, symbol_table: &mut SymbolTable) -> AST {
    match pair.as_rule() {
        Rule::statement => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::expression => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::or_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table);
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table);
                match operator.as_str() {
                    "||" => AST::LogicalOr(Box::new(left), Box::new(right)),
                    _ => panic!("Unexpected logical operator"),
                }
            } else {
                left // 論理演算子がない場合は単純な`and_expr`ノードを返す
            }
        }
        Rule::and_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table);
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table);
                match operator.as_str() {
                    "&&" => AST::LogicalAnd(Box::new(left), Box::new(right)),
                    _ => panic!("Unexpected logical operator"),
                }
            } else {
                left // 論理演算子がない場合は単純な`not_expr`ノードを返す
            }
        }
        Rule::not_expr => {
            let mut inner = pair.into_inner();

            // `not_op` が存在するか確認
            let exist_not_op = if inner.peek().unwrap().as_rule() == Rule::not_op {
                inner.next(); // `not_op` を読み飛ばす
                true
            } else {
                false
            };

            let expr = build_ast(inner.next().unwrap(), symbol_table);

            if exist_not_op {
                AST::LogicalNot(Box::new(expr))
            } else {
                expr // `not_op` が存在しない場合は単純な`comp_expr`ノードを返す
            }
        }
        Rule::comp_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap(), symbol_table);
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap(), symbol_table);
                match operator.as_str() {
                    "==" => AST::Equal(Box::new(left), Box::new(right)),
                    "!=" => AST::NotEqual(Box::new(left), Box::new(right)),
                    "<" => AST::LessThan(Box::new(left), Box::new(right)),
                    "<=" => AST::LessThanOrEqual(Box::new(left), Box::new(right)),
                    ">" => AST::GreaterThan(Box::new(left), Box::new(right)),
                    ">=" => AST::GreaterThanOrEqual(Box::new(left), Box::new(right)),
                    _ => panic!("Unexpected comparison operator"),
                }
            } else {
                left // 比較演算子がない場合は単純な`power`ノードを返す
            }
        }
        Rule::add_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table);

            while let Some(operator) = inner.next() {
                let right = build_ast(inner.next().unwrap(), symbol_table);
                ast = match operator.as_str() {
                    "+" => AST::Add(Box::new(ast), Box::new(right)),
                    "-" => AST::Subtract(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }
            ast
        }
        Rule::mul_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table); // 最初のfactorを取得

            // mul_opとfactorのペアを処理
            while let Some(operator) = inner.next() {
                // 演算子を取得
                let right = build_ast(inner.next().unwrap(), symbol_table); // 右側のfactorを取得
                ast = match operator.as_str() {
                    "*" => AST::Multiply(Box::new(ast), Box::new(right)),
                    "/" => AST::Divide(Box::new(ast), Box::new(right)),
                    "%" => AST::Modulo(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }

            ast
        }
        Rule::pow_expr => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap(), symbol_table);

            while let Some(operator) = inner.next() {
                let right = build_ast(inner.next().unwrap(), symbol_table);
                ast = match operator.as_str() {
                    "^" => AST::PowArcana(Box::new(ast), Box::new(right)),
                    "**" => AST::PowAether(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }

            ast
        }
        Rule::factor => build_ast(pair.into_inner().next().unwrap(), symbol_table),
        Rule::omen => {
            let value = pair.as_str();
            match value {
                "boon" => AST::Omen(true),
                "hex" => AST::Omen(false),
                _ => panic!("Unknown omen: {}", value),
            }
        }
        Rule::arcana => {
            let value = pair.as_str().parse().unwrap();
            AST::Arcana(value)
        }
        Rule::aether => {
            let value = pair.as_str().parse().unwrap();
            AST::Aether(value)
        }
        Rule::rune => {
            let value = pair.as_str().trim_matches('"').to_string();
            AST::Rune(value)
        }
        Rule::forge_var => {
            let mut inner = pair.into_inner();

            let is_morph = if inner.peek().unwrap().as_rule() == Rule::morph {
                inner.next(); // morphを読み飛ばす
                true
            } else {
                false
            };

            let var_name = inner.next().unwrap().as_str().to_string();
            // 型情報を取得して設定
            let var_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => panic!("Unknown type in variable declaration"),
            };

            let value = build_ast(inner.next().unwrap(), symbol_table);

            symbol_table.insert(
                var_name.clone(),
                SymbolInfo {
                    var_type: var_type.clone(),
                    is_morph,
                },
            );

            AST::VarAssign {
                name: var_name,
                value: Box::new(value),
                var_type,
                is_morph,
            }
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
                _ => panic!("Unexpected assignment operator"),
            };
            let value = build_ast(inner.next().unwrap(), symbol_table);

            AST::Assignment {
                name: var_name,
                value: Box::new(value),
                op, // 新しく追加した演算子情報をセット
            }
        }
        Rule::identifier => {
            let var_name = pair.as_str().to_string();

            if let Some(symbol_info) = symbol_table.get(&var_name) {
                AST::Var {
                    name: var_name,
                    var_type: symbol_info.var_type.clone(),
                    is_morph: symbol_info.is_morph,
                }
            } else {
                panic!("Unknown variable: {}", var_name);
            }
        }
        Rule::unveil => {
            let inner = pair.into_inner();
            let args = inner.map(|p| build_ast(p, symbol_table)).collect(); // シンボルテーブルを渡す
            AST::Unveil(args)
        }
        Rule::trans_expr => {
            let mut inner = pair.into_inner();
            let expr = build_ast(inner.next().unwrap(), symbol_table);
            let target_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => panic!("Unknown type in trans expression"),
            };
            AST::Trans(Box::new(expr), target_type)
        }
        _ => {
            panic!("Unexpected rule: {:?}", pair.as_rule())
        }
    }
}
