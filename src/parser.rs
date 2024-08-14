use crate::ast::AST;
use nom::branch::alt;
use nom::character::complete::{char, space0};
use nom::multi::fold_many0;
use nom::sequence::{pair, preceded};
use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn parse_number(input: &str) -> IResult<&str, AST> {
    map_res(preceded(space0, digit1), |s: &str| {
        s.parse::<i64>().map(AST::Number)
    })(input)
}

pub fn parse_term(input: &str) -> IResult<&str, AST> {
    let (input, init) = parse_number(input)?;

    fold_many0(
        pair(preceded(space0, alt((char('*'), char('/')))), parse_number),
        move || init.clone(),
        |acc, (op, val)| {
            if op == '*' {
                AST::Multiply(Box::new(acc), Box::new(val))
            } else {
                AST::Divide(Box::new(acc), Box::new(val))
            }
        },
    )(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, AST> {
    let (input, init) = parse_term(input.trim())?;

    fold_many0(
        pair(preceded(space0, alt((char('+'), char('-')))), parse_term),
        move || init.clone(),
        |acc, (op, val)| {
            if op == '+' {
                AST::Add(Box::new(acc), Box::new(val))
            } else {
                AST::Subtract(Box::new(acc), Box::new(val))
            }
        },
    )(input)
}
