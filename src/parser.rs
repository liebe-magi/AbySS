use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{Type, AST};

#[derive(Parser)]
#[grammar = "abyss.pest"] // 文法ファイルを指定
pub struct AbyssParser;

pub fn parse(input: &str) -> Result<Pair<Rule>, Error<Rule>> {
    match AbyssParser::parse(Rule::statements, input) {
        Ok(mut pairs) => Ok(pairs.next().unwrap()),
        Err(e) => Err(e),
    }
}

pub fn build_ast(pair: Pair<Rule>) -> AST {
    match pair.as_rule() {
        Rule::statement => build_ast(pair.into_inner().next().unwrap()),
        Rule::expression => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap());

            while let Some(operator) = inner.next() {
                let right = build_ast(inner.next().unwrap());
                ast = match operator.as_str() {
                    "+" => AST::Add(Box::new(ast), Box::new(right)),
                    "-" => AST::Subtract(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }

            ast
        }
        Rule::power => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap());

            while let Some(operator) = inner.next() {
                let right = build_ast(inner.next().unwrap());
                ast = match operator.as_str() {
                    "^" => AST::PowArcana(Box::new(ast), Box::new(right)),
                    "**" => AST::PowAether(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }

            ast
        }
        Rule::term => {
            let mut inner = pair.into_inner();
            let mut ast = build_ast(inner.next().unwrap()); // 最初のfactorを取得

            // mul_opとfactorのペアを処理
            while let Some(operator) = inner.next() {
                // 演算子を取得
                let right = build_ast(inner.next().unwrap()); // 右側のfactorを取得
                ast = match operator.as_str() {
                    "*" => AST::Multiply(Box::new(ast), Box::new(right)),
                    "/" => AST::Divide(Box::new(ast), Box::new(right)),
                    _ => unreachable!(),
                };
            }

            ast
        }
        Rule::factor => build_ast(pair.into_inner().next().unwrap()),
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
            let var_name = inner.next().unwrap().as_str().to_string();
            inner.next();
            let value = build_ast(inner.next().unwrap());
            AST::VarAssign(var_name, Box::new(value))
        }
        Rule::identifier => {
            let var_name = pair.as_str().to_string();
            AST::Var(var_name)
        }
        Rule::unveil => {
            let inner = pair.into_inner();
            let args = inner.map(build_ast).collect();
            AST::Unveil(args)
        }
        Rule::trans_expr => {
            let mut inner = pair.into_inner();
            let expr = build_ast(inner.next().unwrap());
            let target_type = match inner.next().unwrap().as_str() {
                "arcana" => Type::Arcana,
                "aether" => Type::Aether,
                "rune" => Type::Rune,
                "omen" => Type::Omen,
                _ => panic!("Unknown type in trans expression"),
            };
            AST::Trans(Box::new(expr), target_type)
        }

        Rule::comp_expr => {
            let mut inner = pair.into_inner();
            let left = build_ast(inner.next().unwrap());
            if let Some(operator_pair) = inner.next() {
                let operator = operator_pair.as_str().to_string();
                let right = build_ast(inner.next().unwrap());
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
        _ => {
            panic!("Unexpected rule: {:?}", pair.as_rule())
        }
    }
}
