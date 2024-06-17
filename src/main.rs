#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

//TODO: maybe parse scss with with grass before and compile to css before lightningcss
//https://docs.rs/grass/latest/grass/

use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

mod css_bundler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    style_sheet: String,
    #[arg(short, long)]
    output: String,
}

//https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code
fn main() -> Result<()> {
    let args = Args::parse();

    let run = css_bundler::run(&args.style_sheet);
    let css = match run {
        Ok(res) => res,
        Err(err) => return Err(Error::new(ErrorKind::Other, err)),
    };
    let output_file = args.output;

    //https://doc.rust-lang.org/std/io/struct.Stdout.html
    // let mut stdout = std::io::stdout().lock();
    // stdout.write_all(&css)?;

    // print!("{css}");

    let p = Path::new(&output_file);

    let mut file = File::create(p)?;
    file.write_all(css.as_bytes())?;

    Ok(())
}
