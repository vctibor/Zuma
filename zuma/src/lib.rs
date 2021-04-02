//! ZUMA is language for creating vector graphics. It is strongly-typed, expressions based.
//! Main target for compilation is SVG, we can also produce PNGs via `resvg` library.
//! For more details see ZUMA language specification in `doc` folder.

#[macro_use] extern crate lalrpop_util;

mod stack;
mod error_handling;

mod parsing;
mod interpretation;
mod code_generation;

mod tests;

use parsing::ZumaParser;
use interpretation::Interpreter;
use code_generation::generate;

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
    pub fn compile(&self, source: &str) -> Result<String> {
        let ast = self.parser.parse(source)?;
        let ir = Interpreter::new().interpret(ast)?;
        let svg = generate(&ir);
        Ok(svg)
    }
}