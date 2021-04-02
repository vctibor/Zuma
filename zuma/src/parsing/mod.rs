//! PARSING -> type checking -> interpretation -> code generation

// because of pub function in grammar.lalrpop
#![allow(dead_code)]

lalrpop_mod!(grammar);

pub mod ast;
pub mod parsing_functions;
mod tests;

use grammar::*;
pub use ast::*;

use anyhow::{Result, anyhow};

use crate::error_handling::ZumaResult;

pub struct ZumaParser {
    parser: DocParser
}

impl ZumaParser {

    pub fn new() -> ZumaParser {
        ZumaParser { parser: DocParser::new() }
    }

    pub fn parse(&self, source: &str) -> Result<Document> {
        match self.parser.parse(source) {
            Ok(doc) => return Ok(doc),
            Err(e) => {
                println!("{}", e);
                return Err(anyhow!("error"));
            }
        }
    }
}