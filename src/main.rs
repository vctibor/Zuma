#[macro_use] extern crate lalrpop_util;

mod parsing;
mod svg_generator;
mod evaluation;

use crate::parsing::ZumaParser;

use std::fs::{read_to_string, File};
use std::time::Instant;
use std::io::Write;
use std::{thread, time};

use anyhow::{Result, bail};

fn compile(parser: &ZumaParser, zuma_source: String) -> Result<String> {
    let parse_res = parser.parse(zuma_source);

    if parse_res.is_none() {
        bail!("Parsing error.");
    }

    let zuma_doc: crate::parsing::ast::Document = parse_res.unwrap();

    // evaluate

    let svg: svg_generator::Document = evaluation::evaluate(zuma_doc);

    Ok(svg.generate())
}

fn compile_file(parser: &ZumaParser) -> Result<()>
{
    let start_time = Instant::now();
    let input = read_to_string("examples/example01.zm")?;
    let svg = compile(&parser, input)?;
    let mut output_file = File::create("examples/example01.svg")?;
    output_file.write_all(svg.as_bytes())?;
    let end_time = start_time.elapsed();
    println!("{:?}", end_time);

    Ok(())
}

fn main() {

    let parser = ZumaParser::new();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        compile_file(&parser);
    }
}
