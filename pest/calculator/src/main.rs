extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct CalcExprParser;

fn main() {
    let parse_result = CalcExprParser::parse(Rule::calc_expr, "1+(2+3)+4* 4");
    dbg!(parse_result.unwrap());
}
