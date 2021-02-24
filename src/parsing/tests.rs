#![allow(unused_imports)]

use crate::ZumaParser;
use crate::parsing::*;
use crate::parsing::ast::*;
use crate::parsing::parsing_functions::color;





#[test]
fn test_number() {
    let parser = NumberParser::new();
    assert!(parser.parse("1234").unwrap() == 1234.0);
    assert!(parser.parse("1234.56").unwrap() == 1234.56);
}

#[test]
fn test_color() {
    let parser = ColorParser::new();
    assert!(parser.parse("black").unwrap() == color(0, 0, 0));
    assert!(parser.parse("white").unwrap() == color(255, 255, 255));
    assert!(parser.parse("red").unwrap() == color(255, 0, 0));
    assert!(parser.parse("green").unwrap() == color(0, 255, 0));
    assert!(parser.parse("blue").unwrap() == color(0, 0, 255));
    assert!(parser.parse("yellow").unwrap() == color(255, 255, 0));
    assert!(parser.parse("#ff00a1").unwrap() == color(255, 0, 161));
    assert!(parser.parse("nonsense").is_err());
}

#[test]
fn test_point() {
    let expected = Point {
        y: Box::new(OperationInput::Literal(Value::Number(0.1))),
        x: Box::new(OperationInput::Literal(Value::Number(5.0)))
    };
    let parser = PointParser::new();
    assert!(parser.parse("[0.1,5]").unwrap() == expected);
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
    assert!(parser.parse(input.to_string()).is_ok());
}

#[test]
fn test_single_line_2() {
    let parser = ZumaParser::new();
    let input = "line start=[0,10] end=[25,50] color=black;";
    assert!(parser.parse(input.to_string()).is_ok());
}

#[test]
fn test_two_lines_1() {
    let parser = ZumaParser::new();
    let input = r#"
        line start=[0,10] end=[25,50] color=black;
        line start=[0,0] end=[50,25] color=red;
    "#;
    assert!(parser.parse(input.to_string()).is_ok());
}

#[test]
fn test_arbitrary_arg_position() {
    let parser = ZumaParser::new();
    let input = r#"
        line color=white end=[100,70] width=11.5 start=[0,40];
        line end=[100,70] width=11.5 color=white start=[0,40];
        line width=11.5 start=[0,40] color=white end=[100,70];
    "#;
    assert!(parser.parse(input.to_string()).is_ok());
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
    assert!(parser.parse(input.to_string()).is_ok());
}


#[test]
fn test_user_procedure_declaration() {
    let parser = ZumaParser::new();
    let input = r#"
    proc name req_arg:Bool reg_arg:Color opt_arg="val" {

    }
    "#.trim();
    let res = parser.parse(input.to_string());
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[test]
fn test_empty_doc() {
    let parser = ZumaParser::new();
    let input = "";
    assert!(parser.parse(input.to_string()).is_ok());
}

#[test]
fn test_empty_scope() {
    let parser = ZumaParser::new();
    let input = r#"{ };"#;
    assert!(parser.parse(input.to_string()).is_ok());
}