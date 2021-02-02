#![allow(dead_code)]        // because of pub function in grammar.lalrpop

lalrpop_mod!(grammar);

pub mod ast;
pub mod parsing_functions;
mod tests;

use grammar::*;
use ast::*;

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

        //println!("{:?}", res);

        res.ok()
    }
}