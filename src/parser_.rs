use crate::ast::AST;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric0, char, space0};
use nom::combinator::recognize;
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::{character::complete::digit1, combinator::map_res, IResult};

fn parse_number(input: &str) -> IResult<&str, AST> {
    map_res(preceded(space0, digit1), |s: &str| {
        s.parse::<i64>().map(AST::Number)
    })(input)
}

fn parse_string(input: &str) -> IResult<&str, AST> {
    let (input, content) = delimited(char('"'), take_until("\""), char('"'))(input.trim())?; // ダブルクォートで囲まれた文字列をパース

    Ok((input, AST::String(content.to_string()))) // 文字列をASTに変換
}

fn parse_factor(input: &str) -> IResult<&str, AST> {
    alt((
        delimited(char('('), parse_expr, char(')')), // 括弧で囲まれた式をパース
        parse_number,
        parse_string,
        parse_var,
    ))(input)
}

fn parse_term(input: &str) -> IResult<&str, AST> {
    let (input, init) = parse_factor(input)?;

    fold_many0(
        pair(preceded(space0, alt((char('*'), char('/')))), parse_factor),
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

fn parse_var_assign(input: &str) -> IResult<&str, AST> {
    let (input, (_, var_name, _, _, value)) = tuple((
        tag("forge"),
        preceded(space0, recognize(pair(alpha1, alphanumeric0))),
        preceded(space0, char(':')),
        preceded(space0, tag("arcana")),
        preceded(space0, preceded(char('='), parse_number)),
    ))(input)?;

    Ok((input, AST::VarAssign(var_name.to_string(), Box::new(value))))
}

fn parse_rune_var_assign(input: &str) -> IResult<&str, AST> {
    let (input, (_, var_name, _, _, value)) = tuple((
        tag("forge"),
        preceded(space0, recognize(pair(alpha1, alphanumeric0))),
        preceded(space0, char(':')),
        preceded(space0, tag("rune")),
        preceded(space0, preceded(char('='), parse_string)),
    ))(input)?;

    Ok((
        input,
        AST::RuneVarAssign(var_name.to_string(), Box::new(value)),
    ))
}

fn parse_var(input: &str) -> IResult<&str, AST> {
    let (input, var_name) = preceded(space0, recognize(pair(alpha1, alphanumeric0)))(input)?;
    Ok((input, AST::Var(var_name.to_string())))
}

fn parse_unveil(input: &str) -> IResult<&str, AST> {
    let (input, (_, _, expr, _)) = tuple((
        preceded(space0, tag("unveil")),
        char('('),
        parse_expr,
        char(')'),
    ))(input)?;

    Ok((input, AST::Unveil(Box::new(expr))))
}

fn parse_expr(input: &str) -> IResult<&str, AST> {
    let input = input.trim();
    let (input, init) = alt((
        parse_rune_var_assign,
        parse_var_assign,
        parse_unveil,
        parse_term,
    ))(input)?;

    let (input, result) = fold_many0(
        pair(preceded(space0, alt((char('+'), char('-')))), parse_term),
        move || init.clone(),
        |acc, (op, val)| {
            if op == '+' {
                AST::Add(Box::new(acc), Box::new(val))
            } else {
                AST::Subtract(Box::new(acc), Box::new(val))
            }
        },
    )(input)?;

    Ok((input, result))
}

pub fn parse_statements(input: &str) -> IResult<&str, Vec<AST>> {
    many0(terminated(parse_expr, preceded(space0, char(';'))))(input.trim())
}
