//! PARSING -> type checking -> interpretation -> code generation

#![allow(dead_code)]        // because of pub function in grammar.lalrpop

lalrpop_mod!(grammar);

pub mod ast;
pub mod parsing_functions;
mod tests;

use grammar::*;
pub use ast::*;

use anyhow::{Result, anyhow};

pub struct ZumaParser {
    parser: DocParser
}

impl ZumaParser {

    pub fn new() -> ZumaParser {
        ZumaParser { parser: DocParser::new() }
    }

    pub fn parse(&self, source: String) -> Result<Document> {
        match self.parser.parse(&source) {
            Ok(document) => Ok(document),
            Err(e) => {
                println!("{:?}", e);
                Err(anyhow!("Parsing error"))
            }
        }
    }
}