use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::AST;

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
        Rule::arcana => {
            let value = pair.as_str().parse().unwrap();
            AST::Arcana(value)
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
        _ => {
            panic!("Unexpected rule: {:?}", pair.as_rule())
        }
    }
}
