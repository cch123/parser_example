#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
pub mod ast;


fn main() {
    let expr = calculator1::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();

    println!("{:#?}", expr);
    //assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

