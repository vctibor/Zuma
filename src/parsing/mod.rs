#![allow(unused_imports)]

lalrpop_mod!(grammar);

pub mod zuma_model;
pub mod parsing_functions;

use crate::parsing::grammar::*;
use crate::parsing::zuma_model::*;

use anyhow::Result;

pub struct ZumaParser {
    parser: PrimParser
}

impl ZumaParser {

    pub fn new() -> ZumaParser {
        ZumaParser { parser: PrimParser::new() }
    }

    /// TODO: Return result
    pub fn parse(&self, source: String) -> Option<GeometricPrimitive> {
        self.parser.parse(&source).ok()
    }
}

#[test]
fn boolean_test1() {
    let parser = BoolParser::new();
    assert!(parser.parse("true").unwrap() == true);
    assert!(parser.parse("false").unwrap() == false);
}

#[test]
fn number_test1() {
    let parser = NumberParser::new();
    assert!(parser.parse("1234").unwrap() == 1234.0);
    assert!(parser.parse("1234.56").unwrap() == 1234.56);
}

#[test]
fn color_test1() {
    let parser = ColorParser::new();
    assert!(parser.parse("black").unwrap() == Color { red: 0, green: 0, blue: 0 });
    assert!(parser.parse("white").unwrap() == Color { red: 255, green: 255, blue: 255 });
    assert!(parser.parse("red").unwrap() == Color { red: 255, green: 0, blue: 0 });
    assert!(parser.parse("green").unwrap() == Color { red: 0, green: 255, blue: 0 });
    assert!(parser.parse("blue").unwrap() == Color { red: 0, green: 0, blue: 255 });
    assert!(parser.parse("yellow").unwrap() == Color { red: 255, green: 255, blue: 0 });
    assert!(parser.parse("#ff00a1").unwrap() == Color { red: 255, green: 0, blue: 161 });
    assert!(parser.parse("nonsense").is_err());
}

#[test]
fn point_test1() {
    let parser = PointParser::new();
    assert!(parser.parse("[0.1,5]").unwrap() == Point { x: 0.1, y: 5.0 });
}

#[test]
fn line_test1() {
    let parser = PrimParser::new();
    let output = Line { start: Point { x: 0., y: 10. }, end: Point { x: 25., y: 50. }, color: Some(Color { red: 0, green: 0, blue: 0 }) };
    let input = "line start=[0,10] end=[25,50] color=black;";
    assert!(parser.parse(input).unwrap() == GeometricPrimitive::Line(output.clone()));
    let input = "line start = [0,10] end = [25,50] color = black;";
    assert!(parser.parse(input).unwrap() == GeometricPrimitive::Line(output));
}

