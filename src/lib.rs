#[macro_use] extern crate lalrpop_util;

mod parsing;
mod svg_generator;
mod evaluation;
mod tests;

use crate::parsing::ZumaParser;

use anyhow::{Result, bail};

pub struct ZumaCompiler {
    parser: ZumaParser
}

impl ZumaCompiler {

    pub fn new() -> ZumaCompiler {
        ZumaCompiler {
            parser: ZumaParser::new()
        }
    }

    pub fn compile(&self, zuma_source: String) -> Result<String> {
        let parse_res = self.parser.parse(zuma_source);
    
        if parse_res.is_none() {
            bail!("Parsing error.");
        }
    
        let zuma_doc: crate::parsing::ast::Document = parse_res.unwrap();
    
        let svg: svg_generator::Document = evaluation::evaluate(zuma_doc)?;
    
        Ok(svg.generate())
    }
}