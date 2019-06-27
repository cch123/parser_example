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
use nom::bytes::complete::{take_until, take_while};
use nom::character::{is_alphanumeric, is_alphabetic};
use nom::character::complete::{space0, not_line_ending};


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
    match comp_expr("a1>b") {
        Ok((s,expr)) => {
            println!("{:#?}", s);
            println!("{:#?}", expr);
        },
        Err(e) => {
            println!("{:#?}", e)
        },
    }
}