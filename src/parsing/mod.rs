#![allow(unused_imports)]

lalrpop_mod!(grammar);

pub mod ast;
pub mod parsing_functions;

use grammar::*;
use ast::*;

use anyhow::Result;

pub struct ZumaParser {
    parser: DocParser
}

impl ZumaParser {

    pub fn new() -> ZumaParser {
        ZumaParser { parser: DocParser::new() }
    }

    /// TODO: Return result
    pub fn parse(&self, source: String) -> Option<Document> {
        let res = self.parser.parse(&source);

        println!("{:?}", res);

        res.ok()
    }
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