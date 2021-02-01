use std::fs::{read_to_string, File};
use std::time::Instant;
use std::io::Write;
use std::{thread, time};

use anyhow::Result;

fn render_svg(svg: String, path: &str) {

    use resvg::render;
    use usvg::{Tree, Options, FitTo};
    use tiny_skia::Pixmap;
    
    

    let mut options = Options::default();
    options.font_family = "Liberation Serif".to_owned();
    
    let tree = Tree::from_str(&svg, &options).unwrap();

    let mut pixmap = Pixmap::new(500, 500).unwrap();

    let mut pixmap_mut = pixmap.as_mut();

    render(&tree, FitTo::Original, pixmap_mut);

    pixmap.save_png(path).unwrap();
}

fn compile_file(parser: &zumalib::ZumaParser) -> Result<()>
{
    let start_time = Instant::now();
    let input = read_to_string("examples/example01.zm")?;
    let svg = zumalib::compile(&parser, input)?;
    let mut output_file = File::create("examples/example01.svg")?;
    output_file.write_all(svg.as_bytes())?;

    render_svg(svg, "examples/example01.png");

    let end_time = start_time.elapsed();
    println!("{:?}", end_time);

    Ok(())
}

fn main() {

    let parser = zumalib::ZumaParser::new();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        let res = compile_file(&parser);

        println!("{:?}", res);

        //break;
    }
}
