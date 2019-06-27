//extern crate nom;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

use nom::{
    IResult,
    character::complete::{digit1 as digit, multispace0 as multispace, alphanumeric0 as alphanumeric},
    bytes::complete::take_till,
    sequence::{preceded, delimited},
    combinator::{map, map_res},
    multi::many0,
    branch::alt,
    bytes::complete::{tag,is_not},
};
use nom::bytes::complete::{take_until, take_while, tag_no_case};
use nom::character::{is_alphanumeric, is_alphabetic};
use nom::character::complete::{space0, not_line_ending};
use nom::sequence::delimitedc;


#[derive(Debug)]
pub enum BoolExpr <'a>{
    CompExpr {
        field : &'a str,
        op : &'a str,
        value : &'a str,
    },
    AndExpr {
        left : Box<BoolExpr<'a>>,
        right : Box<BoolExpr<'a>>,
    },
    OrExpr {
        left : Box<BoolExpr<'a>>,
        right : Box<BoolExpr<'a>>,
    },
    ParenExpr {
        expr: Box<BoolExpr<'a>>
    }
}

// 求优雅写法
fn and_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, left) = comp_expr(i)?;
    let (i, _) = space0(i)?;
    let (i, op) = tag_no_case("and")(i)?;
    let (i, _) = space0(i)?;
    let (i, right) = bool_expr(i)?;
    Ok((i, BoolExpr::AndExpr {left: Box::new(left),right: Box::new(right)}))
}

fn or_expr(i: &str) -> IResult<&str, BoolExpr> {
    let (i, left) = comp_expr(i)?;
    let (i, _) = space0(i)?;
    let (i, op) = tag_no_case("or")(i)?;
    let (i, _) = space0(i)?;
    let (i, right) = bool_expr(i)?;
    Ok((i, BoolExpr::OrExpr{left: Box::new(left),right: Box::new(right)}))
}

fn bool_expr(i: &str) -> IResult<&str, BoolExpr> {
    alt((
        and_expr,
        or_expr,
        comp_expr,
    ))(i)
}

// 看看能不能优化一下，这么写不简洁，还不如以前的 do_parse 宏简单
fn comp_expr(i: &str) -> IResult<&str, BoolExpr> {
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
    Ok((i, BoolExpr::CompExpr {field: left, op: op, value : right}))
}


fn main() {
    //let (s, expr) = comp_expr("a1 = b");
    match bool_expr("a1>=b or a = 1 and c > 832") {
        Ok((s,expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        },
        Err(e) => {
            println!("{:#?}", e)
        },
    }
}