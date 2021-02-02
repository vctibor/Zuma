#![allow(unused_imports)]

use crate::ZumaParser;
use crate::parsing::*;
use crate::parsing::ast::*;

#[test]
fn test_number() {
    let parser = NumberParser::new();
    assert!(parser.parse("1234").unwrap() == 1234.0);
    assert!(parser.parse("1234.56").unwrap() == 1234.56);
}

#[test]
fn test_color() {
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
fn test_point() {
    let parser = PointParser::new();
    assert!(parser.parse("[0.1,5]").unwrap() == Point {y: 0.1, x: 5.0 });
}

#[test]
fn test_string() {
    let parser = TextParser::new();
    let res = parser.parse(r#""Ahoj světe!""#);
    println!("{:?}", res);
    assert!(res.unwrap() == "Ahoj světe!".to_owned());
}

#[test]
fn test_single_line_1() {
    let parser = ZumaParser::new();
    let input = "line start=[0,10] end=[25,50];";
    assert!(parser.parse(input.to_string()).is_some());
}

#[test]
fn test_single_line_2() {
    let parser = ZumaParser::new();
    let input = "line start=[0,10] end=[25,50] color=black;";
    assert!(parser.parse(input.to_string()).is_some());
}

#[test]
fn test_two_lines_1() {
    let parser = ZumaParser::new();
    let input = r#"
        line start=[0,10] end=[25,50] color=black;
        line start=[0,0] end=[50,25] color=red;
    "#;
    assert!(parser.parse(input.to_string()).is_some());
}

#[test]
fn test_arbitrary_arg_position() {
    let parser = ZumaParser::new();
    let input = r#"
        line color=white end=[100,70] width=11.5 start=[0,40];
        line end=[100,70] width=11.5 color=white start=[0,40];
        line width=11.5 start=[0,40] color=white end=[100,70];
    "#;
    assert!(parser.parse(input.to_string()).is_some());
}

#[test]
fn test_constant_declaration() {
    let parser = ZumaParser::new();
    let input = r#"
        let p = [0, 0];
        let c = white;
        let n = 4557;
        let s = "你好！";
    "#;
    assert!(parser.parse(input.to_string()).is_some());
}