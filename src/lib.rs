#[macro_use] extern crate lalrpop_util;

mod parsing;
mod svg_generator;
mod evaluation;

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

#[test]
fn compile_test_rectangle() {
    let input = r#"
rectangle start=[50,50] size=[100,100] color=blue opacity=0.3;
rectangle start=[10,10] size=[100,100] color=red opacity=0.3;
rectangle start=[70,40] size=[100,100] color=green opacity=0.3;
    "#.trim();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(0,0,255);opacity:0.3" width="100" x="50" y="50"/>
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(255,0,0);opacity:0.3" width="100" x="10" y="10"/>
    <rect height="100" style="stroke-width:1;stroke:rgb(0,0,0);fill:rgb(0,255,0);opacity:0.3" width="100" x="70" y="40"/>
</svg>
    "#.trim();

    let compiler = ZumaCompiler::new();
    let res = compiler.compile(input.to_owned()).unwrap();

    assert_eq!(expected, res);
}