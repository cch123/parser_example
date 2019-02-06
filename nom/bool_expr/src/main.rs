#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use nom::multispace;

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

named!(comp_expr<CompleteStr, BoolExpr>, do_parse!(
    field : take_while!(|c : char| c.is_ascii_alphanumeric() && c != '(' && c != ')') >> opt!(complete!(multispace)) >>
    op : take_while!(|c: char| !c.is_ascii_digit()) >> opt!(complete!(multispace)) >>
    value : take_while!(|c: char| c.is_ascii_digit()) >>
    (BoolExpr::CompExpr{field: &field.clone(), op:&op.clone(), value: &value.clone()})
));


named!(paren_expr<CompleteStr, BoolExpr>, do_parse!(
    tag!("(") >>  opt!(complete!(multispace)) >>
    bool_expr : bool_expr >>
    tag!(")") >>  opt!(complete!(multispace)) >>
    (bool_expr)
));

named!(atom<CompleteStr, BoolExpr>, alt!(
    paren_expr | comp_expr
));

named!(pub opt_multispace<CompleteStr, Option<CompleteStr>>,
  opt!(complete!(multispace))
);

named!(and_expr<CompleteStr, BoolExpr>, do_parse!(
    left: atom >> opt_multispace >>
    tag_no_case!("and") >> opt_multispace >>
    right : bool_expr >>
    (BoolExpr::AndExpr{left: Box::new(left), right: Box::new(right)})
));

named!(or_expr<CompleteStr, BoolExpr>, do_parse!(
    left: atom >> opt_multispace >>
    tag_no_case!("or") >> opt_multispace >>
    right : bool_expr >>
    (BoolExpr::AndExpr{left: Box::new(left), right: Box::new(right)})
));

named!(bool_expr<CompleteStr, BoolExpr>, alt!(
    and_expr | or_expr | atom
));

fn main() {
    let ex = "a = 1 and b = 2 and c =3";
    println!("{:#?}", bool_expr(CompleteStr(ex)));
    let ex = "(a = 1 and b = 2) and c =3";
    println!("{:#?}", bool_expr(CompleteStr(ex)));
}