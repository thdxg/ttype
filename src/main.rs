mod app;

use anyhow::{Context, Result};
use std::{env::args, fs::File, io::Read};

use crate::app::App;

fn main() -> Result<()> {
    if let Some(fpath) = args().nth(1) {
        let mut f = File::open(fpath).context("Failed to open file")?;
        let mut text: String = String::new();
        f.read_to_string(&mut text)
            .context("Failed to read content")?;

        let app = App::new(text);
        app.run()
    } else {
        println!("ttype: a simple typing test tui");
        println!("usage:\n");
        println!("\tttype <file-path>");
        Ok(())
    }
}
