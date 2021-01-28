#[macro_use] extern crate lalrpop_util;

mod parsing;
mod tests;
mod svg_generator;
mod translator;

use crate::grammar::ExprParser;
use crate::grammar::PrimParser;

use crate::translator::translate;

use std::time::Instant;

lalrpop_mod!(pub grammar);

fn main() {

    let start_time = Instant::now();

    let input = "line [0,10] [25,50] #ff001a;";

    let parser = PrimParser::new();

    let parse_res = parser.parse(input);

    let doc = parsing::zuma_model::Document { primitives: vec!(parse_res.unwrap()) };

    // evaluate

    let svg_model = translate(doc);

    let svg = svg_generator::generate_svg(svg_model);

    println!("{}", svg);

    let end_time = start_time.elapsed();

    println!("{:?}", end_time);
}
