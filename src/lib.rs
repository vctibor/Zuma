//! ZUMA is language for creating vector graphics. It is strongly-typed, expressions based.
//! Main target for compilation is SVG, we can also produce PNGs via `resvg` library.
//! For more details see ZUMA language specification in `doc` folder.

#[macro_use] extern crate lalrpop_util;

mod parsing;
mod interpretation;
mod code_generation;

mod tests;

use parsing::ZumaParser;

use code_generation::generate;

use anyhow::{Result, bail};

pub struct ZumaCompiler {
    parser: ZumaParser
}

impl ZumaCompiler {

    /// Instantiate ZUMA compiler.
    pub fn new() -> ZumaCompiler {
        ZumaCompiler {
            parser: ZumaParser::new()
        }
    }

    /// Translates ZUMA source code into SVG.
    pub fn compile(&self, zuma_source: String) -> Result<String> {
        let parse_res = self.parser.parse(zuma_source);
    
        if parse_res.is_none() {
            bail!("Parsing error.");
        }
    
        let zuma_doc: crate::parsing::ast::Document = parse_res.unwrap();
    
        let graphics = interpretation::interpret(zuma_doc)?;

        let svg = generate(graphics);

        Ok(svg)
    }
}