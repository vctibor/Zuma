use std::fs::{read_to_string, File};
use std::time::Instant;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use structopt::StructOpt;

/// ZUMA language compiler.
#[derive(StructOpt, Debug)]
#[structopt(
    name = "zuma",
    about = "ZUMA language compiler",
    rename_all = "kebab-case"
)]
struct Args {
    
    /// Don't exit, recompile whenever source changes.
    #[structopt(short, long)]
    watch: bool,

    /// Create .png output file.
    #[structopt(long, parse(try_from_str), default_value="false")]
    png: bool,

    /// Create .svg output file. 
    #[structopt(long, parse(try_from_str), default_value="true")]
    svg: bool,

    /// Output to STDOUT.
    #[structopt(long, parse(try_from_str), default_value="false")]
    stdout: bool,

    /// Path to source file.
    #[structopt(parse(from_os_str))]
    filename: PathBuf,

    /// Optional custom name for .png output.
    /// Implies png = true.
    #[structopt(parse(from_os_str))]
    png_filename: Option<PathBuf>,

    /// Optional custom name for .svg output.
    /// Implies svg = true.
    #[structopt(parse(from_os_str))]
    svg_filename: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> Result<()> {

    let file_stem = args.filename.file_stem().ok_or(anyhow!("Invalid filename!"))?;

    let svg_filename = {
        let mut svg_filename = file_stem.clone().to_owned();
        svg_filename.push(".svg");
        args.svg_filename.clone().unwrap_or(PathBuf::from(svg_filename))
    };

    let png_filename = {
        let mut png_filename = file_stem.clone().to_owned();
        png_filename.push(".png");
        args.png_filename.clone().unwrap_or(PathBuf::from(png_filename))
    };

    let start_time = Instant::now();

    let compiler = zumalib::ZumaCompiler::new();

    let input = read_to_string(args.filename)?;
    let svg = compiler.compile(&input)?;

    if args.stdout {
        println!("{}", svg);
    }

    if args.svg || args.svg_filename.is_some() {
        let mut output_file = File::create(svg_filename)?;
        output_file.write_all(svg.as_bytes())?;
    }

    if args.png || args.png_filename.is_some() {
        println!("Sorry, PNG rendering currently isn't available!");
        /*
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
        */
    }

    let end_time = start_time.elapsed();
    println!("{:?}", end_time);

    Ok(())
}
