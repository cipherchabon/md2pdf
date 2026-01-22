use clap::Parser;
use md2pdf_rs::{convert_file, Config};
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "md2pdf")]
#[command(author, version, about = "Convert Markdown to PDF using Typst", long_about = None)]
struct Cli {
    /// Input Markdown file
    input: PathBuf,

    /// Output PDF file (defaults to input filename with .pdf extension)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Paper size (a4, letter, legal)
    #[arg(long, default_value = "a4")]
    paper: String,

    /// Theme to use (default, github, academic, minimal)
    #[arg(long, default_value = "default")]
    theme: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let output = cli.output.unwrap_or_else(|| {
        let mut out = cli.input.clone();
        out.set_extension("pdf");
        out
    });

    let config = Config {
        paper_size: cli.paper,
        theme: cli.theme,
        verbose: cli.verbose,
    };

    if cli.verbose {
        eprintln!("Converting {} to {}", cli.input.display(), output.display());
    }

    match convert_file(&cli.input, &output, &config) {
        Ok(()) => {
            if cli.verbose {
                eprintln!("Successfully created {}", output.display());
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}
