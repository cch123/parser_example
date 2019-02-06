#[macro_use]
extern crate nom;
use nom::multispace;
use nom::types::CompleteStr;

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
}

named!(comp_expr<CompleteStr, BoolExpr>, do_parse!(
    opt_multispace >>
    field : take_while!(|c : char| c.is_ascii_alphanumeric() && c != '(' && c != ')') >> opt!(complete!(multispace)) >>
    op : take_while!(|c: char| !c.is_ascii_digit() && !c.is_whitespace()) >> opt!(complete!(multispace)) >>
    value : take_while!(|c: char| c.is_ascii_digit()) >>
    opt_multispace >>
    (BoolExpr::CompExpr{field: &field.clone(), op:&op.clone(), value: &value.clone()})
));

named!(paren_expr<CompleteStr, BoolExpr>, do_parse!(
    opt_multispace >>
    tag!("(") >>  opt_multispace >>
    bool_expr : bool_expr >>
    tag!(")") >>  opt_multispace >>
    (bool_expr)
));

named!(atom<CompleteStr, BoolExpr>, alt!(
    paren_expr | comp_expr
));

named!(pub opt_multispace<CompleteStr, Option<CompleteStr>>,
  opt!(complete!(multispace))
);

named!(bool_expr<CompleteStr, BoolExpr>, do_parse!(
    initial : atom >> opt_multispace >>
    res : fold_many0!(
        pair!(alt!(tag_s!("and") | tag_s!("or")), atom),
        initial,
        |acc, (op , val)| {
            match (op as CompleteStr).as_ref() {
                "and" => BoolExpr::AndExpr{left: Box::new(acc), right: Box::new(val)},
                "or" => BoolExpr::OrExpr{left: Box::new(acc), right: Box::new(val)},
                _ => unreachable!()
            }
        }
    ) >> (res)
));

/*
named!(bool_expr<CompleteStr, BoolExpr>, do_parse!(
    initial : atom >> opt_multispace >>
    remainder : many0!(
        alt!(
            do_parse!(tag!("and") >> opt_multispace >> and : atom >> opt_multispace >> ("and", and) ) |
            do_parse!(tag!("or") >> opt_multispace >> or : atom >> opt_multispace >> ("or", or)  )
        )
    ) >>
    (fold_exprs(initial, remainder))
));

fn fold_exprs<'a>(initial: BoolExpr<'a>, remainder: Vec<(&'a str, BoolExpr<'a>)>) -> BoolExpr<'a> {
    println!("initial: {:#?}, remainder: {:#?}", &initial, &remainder);
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            "and" => BoolExpr::AndExpr{left: Box::new(acc), right: Box::new(expr)},
            "or" => BoolExpr::OrExpr{left: Box::new(acc), right: Box::new(expr)},
            _ => unreachable!()
        }
    })
}
*/

fn main() {
    let ex = "a = 1 and b = 2 and c =3";
    println!("{:#?}", bool_expr(CompleteStr(ex)));
    let ex = "(a = 1 and b = 2) and c =3";
    println!("{:#?}", bool_expr(CompleteStr(ex)));
    let ex = "a = 1 and (b = 2 and c =   3)";
    println!("{:#?}", bool_expr(CompleteStr(ex)));
}
