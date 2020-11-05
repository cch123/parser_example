use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

use nom::bytes::complete::{tag_no_case, take_until, take_while};
use nom::character::complete::{not_line_ending, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::sequence::{delimitedc, tuple};
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

#[derive(Debug)]
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

// 求优雅写法
fn and_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, (_, left, _, _, _, right, _)) = tuple((
        space0,
        atom,
        space0,
        tag_no_case("and"),
        space0,
        bool_expr,
        space0,
    ))(i)?;
    Ok((
        i,
        BoolExpr::AndExpr {
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}

fn or_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, (_, left, _, _, _, right, _)) = tuple((
        space0,
        atom,
        space0,
        tag_no_case("or"),
        space0,
        bool_expr,
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
    alt((and_expr, or_expr, atom))(i)
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
        is_not(" \t\r\nabcdefghijklmnopqrstuvwxyz"),
        space0,
        alphanumeric,
        space0,
    ))(i)?;
    /*
    let (i, left) = alphanumeric(i)?;
    // 吃掉 space
    let (i, _) = space0(i)?;
    //let (i,op)  = tag("=")(i)?;
    //let (i, op) = take_till(|c: char| c.is_alphabetic())(i)?;
    //let (i, op) = take_till(|c: char| c.is_alphabetic())(i)?;
    let (i, op) = is_not(" \t\r\nabcdefghijklmnopqrstuvwxyz")(i)?;
    // 吃掉 space
    let (i, _) = space0(i)?;
    let (i, right) = alphanumeric(i)?;
    */
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
    //let (s, expr) = comp_expr("a1 = b");
    match bool_expr(" ( a1 >=b or a= 1) and c > 832 ") {
        Ok((s, expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        }
        Err(e) => println!("{:#?}", e),
    }
}