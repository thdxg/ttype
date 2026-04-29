mod app;

use anyhow::{Context, Result};
use std::{env::args, fs::File, io::Read};

use crate::app::App;

fn main() -> Result<()> {
    let fpath = args().nth(1).context("file path must be specified")?;
    let mut f = File::open(fpath)?;
    let mut text: String = String::new();
    f.read_to_string(&mut text)?;

    let mut app = App::new(text);
    app.run()
}
