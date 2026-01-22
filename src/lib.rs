pub mod config;
pub mod parser;
pub mod renderer;
pub mod transpiler;
pub mod utils;

use std::fs;
use std::path::Path;
use thiserror::Error;

pub use config::Config;
use parser::{frontmatter::Frontmatter, markdown::parse_markdown};
use renderer::pdf::render_pdf;
use transpiler::typst::to_typst;

#[derive(Error, Debug)]
pub enum Md2PdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Typst compilation error: {0}")]
    Typst(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Md2PdfError>;

/// Convert a Markdown file to PDF
pub fn convert_file(input: &Path, output: &Path, config: &Config) -> Result<()> {
    let content = fs::read_to_string(input)?;
    let pdf_bytes = convert(&content, config)?;
    fs::write(output, pdf_bytes)?;
    Ok(())
}

/// Convert Markdown content to PDF bytes
pub fn convert(markdown: &str, config: &Config) -> Result<Vec<u8>> {
    let (frontmatter, content) = Frontmatter::extract(markdown)?;
    let events = parse_markdown(content);
    let typst_code = to_typst(events, &frontmatter, config);
    let pdf = render_pdf(&typst_code, config)?;
    Ok(pdf)
}
