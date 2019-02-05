#![recursion_limit = "1024"]

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde_json;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn convert(
    query: String
) -> Result<Expression, ParseError> {
    let parse_result = ExprParser::parse(Rule::expr, query.as_str());
    match parse_result {
        Ok(mut expr_ast) => {
            let ast = generate_ast(expr_ast.next().unwrap());
            return Ok(ast);
        }
        Err(err) => {
            // TODO: more friendly error
            Err(ParseError {
                location: err.location,
                expected: "".to_string(),
            })
        }
    }
}

use pest::iterators::Pair;

#[derive(Debug)]
pub enum Expression {
    CompExpr(String, String, String),
    AndExpr(Box<Expression>, Box<Expression>),
    OrExpr(Box<Expression>, Box<Expression>),
}

fn generate_ast(pair: Pair<Rule>) -> Expression {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::and_op, Assoc::Left) | Operator::new(Rule::or_op, Assoc::Left),
    ]);

    consume(pair, &climber)
}

fn consume(pair: Pair<Rule>, climber: &PrecClimber<Rule>) -> Expression {
    let atom = |pair| consume(pair, climber);
    let infix = |lhs, op: Pair<Rule>, rhs| match op.as_rule() {
        Rule::and_op => Expression::AndExpr(Box::new(lhs), Box::new(rhs)),
        Rule::or_op => Expression::OrExpr(Box::new(lhs), Box::new(rhs)),
        _ => unreachable!(),
    };

    match pair.as_rule() {
        Rule::expr => {
            let pairs = pair.into_inner();
            climber.climb(pairs, atom, infix)
        }
        Rule::paren_bool => pair.into_inner().next().map(atom).unwrap(),
        Rule::comp_expr => {
            let mut iter = pair.into_inner();
            let (lhs, op, rhs) = (
                iter.next().unwrap().as_str().to_string(),
                iter.next().unwrap().as_str().to_string(),
                iter.next().unwrap().as_str().to_string(),
            );
            return Expression::CompExpr(lhs, op, rhs);
        }
        _ => unreachable!(),
    }
}

fn main() {
    let str = "a = 1 and b = 2 and c = 3".to_string();
    let res = convert(str);
    println!("{:?}", res.unwrap());
}