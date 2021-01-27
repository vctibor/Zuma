#[macro_use] extern crate lalrpop_util;

mod ast;


use crate::grammar::ExprParser;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn main() {

    let expr = ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();

        println!("{:?}", expr);
}
