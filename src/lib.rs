#[macro_use] extern crate lalrpop_util;

mod parsing;
mod svg_generator;
mod evaluation;

pub use crate::parsing::ZumaParser;

use anyhow::{Result, bail};

pub fn compile(parser: &ZumaParser, zuma_source: String) -> Result<String> {
    let parse_res = parser.parse(zuma_source);

    if parse_res.is_none() {
        bail!("Parsing error.");
    }

    let zuma_doc: crate::parsing::ast::Document = parse_res.unwrap();

    let svg: svg_generator::Document = evaluation::evaluate(zuma_doc)?;

    Ok(svg.generate())
}