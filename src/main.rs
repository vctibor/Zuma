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
    options.font_family = "Fira Code".to_owned();
    
    let tree = Tree::from_str(&svg, &options).unwrap();
    let mut pixmap = Pixmap::new(1000, 1000).unwrap();
    let pixmap_mut = pixmap.as_mut();
    render(&tree, FitTo::Original, pixmap_mut);
    pixmap.save_png(path).unwrap();
}

fn compile_file(compiler: &zumalib::ZumaCompiler) -> Result<()>
{
    let folder = "examples";
    //let input_file = "front_page01";
    let input_file = "example01";

    let zuma_input = format!("{}/{}.zm", folder, input_file);
    let svg_output = format!("{}/{}.svg", folder, input_file);
    let png_output = format!("{}/{}.png", folder, input_file);

    let start_time = Instant::now();
    let input = read_to_string(zuma_input)?;
    let svg = compiler.compile(input)?;
    let mut output_file = File::create(svg_output)?;
    output_file.write_all(svg.as_bytes())?;

    //render_svg(svg, &png_output);

    let end_time = start_time.elapsed();
    println!("{:?}", end_time);

    Ok(())
}

fn main() {

    let compiler = zumalib::ZumaCompiler::new();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        let res = compile_file(&compiler);

        println!("{:?}", res);
        
        println!("");

        //break;
    }
}
