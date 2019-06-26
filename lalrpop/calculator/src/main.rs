#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
pub mod ast;

fn walk_expr(expr: ast::Expr) -> i32 {
    match expr {
        ast::Expr::Number(n) => return n,
        ast::Expr::Op(l, o, r) => {
            match o {
                ast::Opcode::Add => {
                    return walk_expr(*l) + walk_expr(*r);
                },
                ast::Opcode::Sub => {
                    return walk_expr(*l) - walk_expr(*r);
                },
                ast::Opcode::Mul => {
                    return walk_expr(*l) * walk_expr(*r);
                },
                ast::Opcode::Div => {
                    return walk_expr(*l) / walk_expr(*r);
                },
            }
        }, //return walk_expr(l) + ,
        ast::Expr::Error => println!("fuck"),
    }
    0
}

fn main() {
    let expr = calculator1::ExprParser::new()
        .parse("10 * 20 +2")
        .unwrap();

    dbg!(&expr);

    println!("result is : {}", walk_expr(*expr));
}

//assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
