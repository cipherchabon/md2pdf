# md2pdf

A CLI tool to convert Markdown to PDF using [Typst](https://typst.app/) as the rendering backend.

## Features

- **Markdown parsing** with pulldown-cmark (GFM tables, code blocks, math, task lists, strikethrough)
- **YAML frontmatter** support (title, author, date)
- **4 predefined themes**: default, github, academic, minimal
- **Multiple paper sizes**: A4, Letter, Legal
- **Math support** using Typst syntax
- **Bundled fonts** for consistent rendering across platforms

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap cipherchabon/tap
brew install md2pdf-rs
```

### Cargo (crates.io)

```bash
cargo install md2pdf-rs
```

### From source

```bash
git clone https://github.com/cipherchabon/md2pdf
cd md2pdf
cargo install --path .
```

## Usage

```bash
# Basic usage - creates input.pdf
md2pdf input.md

# Specify output file
md2pdf input.md -o output.pdf

# Use a different theme
md2pdf input.md --theme github

# Change paper size
md2pdf input.md --paper letter

# Verbose output
md2pdf input.md -v
```

### Options

```
Usage: md2pdf [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input Markdown file

Options:
  -o, --output <OUTPUT>  Output PDF file (defaults to input filename with .pdf extension)
      --paper <PAPER>    Paper size (a4, letter, legal) [default: a4]
      --theme <THEME>    Theme to use (default, github, academic, minimal) [default: default]
  -v, --verbose          Enable verbose output
  -h, --help             Print help
  -V, --version          Print version
```

## Supported Markdown Features

- Headings (H1-H6)
- Paragraphs
- **Bold** and *italic* text
- ~~Strikethrough~~
- `Inline code` and code blocks with syntax highlighting
- Links and images
- Ordered and unordered lists
- Task lists
- Blockquotes
- Horizontal rules
- GFM tables with alignment
- Math (Typst syntax): `$E = m c^2$`

## Frontmatter

Add YAML frontmatter to customize the document header:

```yaml
---
title: My Document
author: John Doe
date: 2025-01-21
---

# Content starts here...
```

## Themes

| Theme | Description |
|-------|-------------|
| `default` | Clean, readable style with justified text |
| `github` | GitHub-flavored markdown style |
| `academic` | Formal style with numbered headings |
| `minimal` | Simple, distraction-free design |

## Math Support

Math uses Typst syntax (not LaTeX). Key differences:

```markdown
Inline: $E = m c^2$           (note: spaces between variables)
Block:  $$ sum_(i=1)^n x_i $$  (use sum_() not \sum_{})
```

See the [Typst math documentation](https://typst.app/docs/reference/math/) for full syntax.

## License

MIT
