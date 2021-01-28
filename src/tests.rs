use crate::ExprParser;
use crate::grammar::PrimParser;

use crate::ast::GeometricPrimitive;
use crate::ast::Point;
use crate::ast::Line;
use crate::ast::Expr;
use crate::ast::Color;

#[test]
fn boolean_test1() {
    let parser = ExprParser::new();
    assert!(parser.parse("true").unwrap() == Expr::Boolean(true));
    assert!(parser.parse("false").unwrap() == Expr::Boolean(false));
}

#[test]
fn number_test1() {
    let parser = ExprParser::new();
    assert!(parser.parse("1234").unwrap() == Expr::Number(1234.0));
    assert!(parser.parse("1234.56").unwrap() == Expr::Number(1234.56));
}

#[test]
fn color_test1() {
    let parser = ExprParser::new();
    assert!(parser.parse("black").unwrap() == Expr::Color(Color { red: 0, green: 0, blue: 0 }));
    assert!(parser.parse("white").unwrap() == Expr::Color(Color { red: 255, green: 255, blue: 255 }));
    assert!(parser.parse("red").unwrap() == Expr::Color(Color { red: 255, green: 0, blue: 0 }));
    assert!(parser.parse("green").unwrap() == Expr::Color(Color { red: 0, green: 255, blue: 0 }));
    assert!(parser.parse("blue").unwrap() == Expr::Color(Color { red: 0, green: 0, blue: 255 }));
    assert!(parser.parse("yellow").unwrap() == Expr::Color(Color { red: 255, green: 255, blue: 0 }));
    assert!(parser.parse("#ff00a1").unwrap() == Expr::Color(Color { red: 255, green: 0, blue: 161 }));
    assert!(parser.parse("nonsense").is_err());
}

#[test]
fn point_test1() {
    let parser = ExprParser::new();
    assert!(parser.parse("[0.1,5]").unwrap() == Expr::Point( Point { x: 0.1, y: 5.0 }));
}

/*
#[test]
fn primitive_test1() {
    let parser = PrimParser::new();
    assert!(parser.parse("line [0,10] [25,50];").unwrap() == GeometricPrimitive::Line);
}
*/

#[test]
fn line_test1() {
    let parser = PrimParser::new();
    let input = "line [0,10] [25,50];";
    let output = Line { start: Point { x: 0., y: 10. }, end: Point { x: 25., y: 50. } };
    assert!(parser.parse(input).unwrap() == GeometricPrimitive::Line(output));
}