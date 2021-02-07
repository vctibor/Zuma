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
use interpretation::interpret;

use anyhow::Result;

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
        Ok(
            generate(
                interpret(
                    self.parser.parse(zuma_source)?
                )?
            )
        )
    }
}