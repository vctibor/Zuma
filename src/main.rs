#[macro_use] extern crate lalrpop_util;

mod ast;
mod tests;

use crate::grammar::ExprParser;

lalrpop_mod!(pub grammar);

fn main() {

    let expr = ExprParser::new()
        .parse("false")
        .unwrap();

    println!("{:?}", expr);

    let res = ast::parse_color("#ff00a1".to_owned());
    println!("{:?}", res);
}
