#[macro_use] extern crate lalrpop_util;

mod ast;
mod tests;
mod generator;
mod translator;

use crate::grammar::ExprParser;

lalrpop_mod!(pub grammar);

fn main() {

    let input = "line [0,10] [25,50];";

    let parser = PrimParser::new();

    //generator::generate_svg()
}
