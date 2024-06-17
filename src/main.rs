use tempdir::TempDir;
use std::process::Command;
use std::error::Error;
use clap::Parser;

use crate::parse::parse;
use crate::writer::write_files;
mod download;
mod filetype;
mod segment;
mod segment_optimizer;
mod parse;
mod writer;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    // Which file to consume as input, currently only supports Markdown
    mdfile: String,

    // Which command to run within the temp directory
    #[arg(default_value = "bash")]
    command: String
}

impl CliArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let md_text = download::get_by_path(&self.mdfile)?;
        let segments = parse(&md_text);
        let tmp = TempDir::new("glu")?;
        write_files(&tmp, &segments)?;

        Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .current_dir(&tmp)
            .spawn()?
            .wait()?;

        Ok(())
    }
}

fn main() {
    let cli = CliArgs::parse();
    cli.run().expect("Failed to run");
}
