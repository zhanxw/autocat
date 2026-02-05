use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to read. If empty, read from stdin.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn process_reader<R: Read>(reader: R) -> Result<()> {
    let mut reader = reader;
    let mut stdout = io::stdout().lock();
    io::copy(&mut reader, &mut stdout).context("Failed to copy content to stdout")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.files.is_empty() {
        // Read from stdin
        let stdin = io::stdin().lock();
        let (reader, _compression) = niffler::get_reader(Box::new(stdin))
            .context("Failed to detect compression from stdin")?;
        process_reader(reader)?;
    } else {
        for file_path in args.files {
            let file = File::open(&file_path)
                .with_context(|| format!("Failed to open file: {:?}", file_path))?;
            let (reader, _compression) = niffler::get_reader(Box::new(file))
                .with_context(|| format!("Failed to detect compression for file: {:?}", file_path))?;
            process_reader(reader)?;
        }
    }

    Ok(())
}
