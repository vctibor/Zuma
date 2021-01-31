use std::fs::{read_to_string, File};
use std::time::Instant;
use std::io::Write;
use std::{thread, time};

use anyhow::Result;

fn compile_file(parser: &zumalib::ZumaParser) -> Result<()>
{
    let start_time = Instant::now();
    let input = read_to_string("examples/example01.zm")?;
    let svg = zumalib::compile(&parser, input)?;
    let mut output_file = File::create("examples/example01.svg")?;
    output_file.write_all(svg.as_bytes())?;
    let end_time = start_time.elapsed();
    println!("{:?}", end_time);

    Ok(())
}

fn main() {

    let parser = zumalib::ZumaParser::new();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        let _ = compile_file(&parser);

        //println!("{:?}", res);

        //break;
    }
}
