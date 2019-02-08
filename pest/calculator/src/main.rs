extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct CalcExprParser;

#[derive(Debug)]
enum Expression {
    Num(String),
    AddExpr(Box<Expression>, Box<Expression>),
    MinusExpr(Box<Expression>, Box<Expression>),
    MulExpr(Box<Expression>, Box<Expression>),
    DivExpr(Box<Expression>, Box<Expression>),
}

fn main() {
    let parse_result = CalcExprParser::parse(Rule::calc_expr, "1+(2+3)+4* 4");
    let ast = generate_ast(parse_result.unwrap().next().unwrap());
    println!("{:#?}", ast);

    let parse_result = CalcExprParser::parse(Rule::calc_expr, "(1+2+3+4)*5");
    let ast = generate_ast(parse_result.unwrap().next().unwrap());
    println!("{:#?}", ast);

    let parse_result = CalcExprParser::parse(Rule::calc_expr, "(-1+2+3+4)*5");
    let ast = generate_ast(parse_result.unwrap().next().unwrap());
    println!("{:#?}", ast);
}

fn generate_ast(pair: Pair<Rule>) -> Expression {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
        Operator::new(Rule::mul, Assoc::Left) | Operator::new(Rule::div, Assoc::Left),
    ]);

    consume(pair.into_inner().next().unwrap(), &climber)
}

fn consume(pair: Pair<Rule>, climber: &PrecClimber<Rule>) -> Expression {
    let atom = |pair| consume(pair, climber);
    let infix = |lhs, op: Pair<Rule>, rhs| match op.as_rule() {
        Rule::add => return Expression::AddExpr(Box::new(lhs), Box::new(rhs)),
        Rule::minus => return Expression::MinusExpr(Box::new(lhs), Box::new(rhs)),
        Rule::mul => return Expression::MulExpr(Box::new(lhs), Box::new(rhs)),
        Rule::div => return Expression::DivExpr(Box::new(lhs), Box::new(rhs)),
        _ => unreachable!(),
    };

    match pair.as_rule() {
        Rule::expr => climber.climb(pair.into_inner(), atom, infix),
        Rule::paren => pair.into_inner().next().map(atom).unwrap(),
        Rule::num_lit => Expression::Num ( pair.as_str().to_string() ),
        _ => unreachable!()
    }
}
