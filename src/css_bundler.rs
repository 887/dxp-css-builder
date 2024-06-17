//https://docs.rs/lightningcss/1.0.0-alpha.41/lightningcss/bundler/index.html

use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::{ParserOptions, PrinterOptions},
};
use std::path::Path;

pub fn run(stylesheet: &str) -> Result<String, String> {
    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
    let stylesheet_maybe = bundler.bundle(Path::new(stylesheet));
    let stylesheet = match stylesheet_maybe {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    let css_maybe = stylesheet.to_css(PrinterOptions::default());
    let css = match css_maybe {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    let code = css.code;
    Ok(code)
}
