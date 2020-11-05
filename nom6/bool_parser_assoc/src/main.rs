//extern crate nom;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

use nom::bytes::complete::{tag_no_case, take_until, take_while};
use nom::character::complete::{not_line_ending, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::multi::fold_many0;
use nom::sequence::{delimitedc, pair, tuple};
use nom::{
    branch::alt,
    bytes::complete::take_till,
    bytes::complete::{is_not, tag},
    character::complete::{
        alphanumeric0 as alphanumeric, digit1 as digit, multispace0 as multispace,
    },
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, Clone)]
pub enum BoolExpr<'a> {
    CompExpr {
        field: &'a str,
        op: &'a str,
        value: &'a str,
    },
    AndExpr {
        left: Box<BoolExpr<'a>>,
        right: Box<BoolExpr<'a>>,
    },
    OrExpr {
        left: Box<BoolExpr<'a>>,
        right: Box<BoolExpr<'a>>,
    },
    ParenExpr {
        expr: Box<BoolExpr<'a>>,
    },
}

// a = 1 and b = 3 and d = 4
fn and_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, init) = atom(i)?;
    fold_many0(
        pair(tag_no_case("and"), atom),
        init,
        |acc, (op, val): (&str, BoolExpr)| {
            return BoolExpr::AndExpr {
                left: Box::new(acc),
                right: Box::new(val),
            };
        },
    )(i)
}

fn or_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, init) = atom_or_expr(i)?;
    fold_many0(
        pair(tag_no_case("or"), alt((atom_or_expr, atom))),
        init,
        |acc, (op, val): (&str, BoolExpr)| {
            return BoolExpr::OrExpr {
                left: Box::new(acc),
                right: Box::new(val),
            };
        },
    )(i)
}

fn atom_or_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, (_, left, _, _, _, right, _)) = tuple((
        space0,
        alt((and_expr, comp_expr)),
        space0,
        tag_no_case("or"),
        space0,
        alt((and_expr, comp_expr)),
        space0,
    ))(i)?;

    Ok((
        i,
        BoolExpr::OrExpr {
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}

fn paren_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, (_, _, _, expr, _, _, _)) = tuple((
        space0,
        tag("("),
        space0,
        bool_expr,
        space0,
        tag(")"),
        space0,
    ))(i)?;

    Ok((i, expr))
}

fn bool_expr(i: &str) -> IResult<&str, BoolExpr> {
    alt((or_expr, and_expr, comp_expr))(i)
}

fn atom(i: &str) -> IResult<&str, BoolExpr> {
    alt((paren_expr, comp_expr))(i)
}

fn comp_expr(i: &str) -> IResult<&str, BoolExpr> {
    // 优化写法，和下面的是等价的
    let (i, (_, left, _, op, _, right, _)) = tuple((
        space0,
        alphanumeric,
        space0,
        is_not(" \t\r\nabcdefghijklmnopqrstuvwxyz1234567890"),
        space0,
        alphanumeric,
        space0,
    ))(i)?;

    Ok((
        i,
        BoolExpr::CompExpr {
            field: left,
            op: op,
            value: right,
        },
    ))
}

fn main() {
    match bool_expr(" d= 1 and a1 >=b or a= 1 and c > 832 ") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("x= 1 and d= 1 and a1 >=b or a= 1 and c > 832 ") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("d=1 and a1 >=b or a= 1 and c > 832 ") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("d=1 and (a1 >=b or a= 1) and c > 832 ") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1 and b = 2 and c = 3") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1 and b = 2 and c = 3 and d= 5") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1 and b = 2 or c = 3 and d= 5") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1 or b = 2 or d= 5") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1 or b = 2") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }

    match bool_expr("a = 1") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }
}

