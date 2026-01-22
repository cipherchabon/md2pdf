use crate::config::themes::get_theme_preamble;
use crate::config::Config;
use crate::parser::frontmatter::Frontmatter;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Tag, TagEnd};

pub fn to_typst(events: Vec<Event<'_>>, frontmatter: &Frontmatter, config: &Config) -> String {
    let mut converter = TypstConverter::new(config);
    converter.convert(events, frontmatter)
}

struct TypstConverter<'a> {
    config: &'a Config,
    output: String,
    list_stack: Vec<ListContext>,
    in_table: bool,
    table_alignments: Vec<Alignment>,
    table_row: Vec<String>,
    current_cell: String,
    in_heading: bool,
    in_emphasis: bool,
    in_strong: bool,
    in_strikethrough: bool,
    in_link: bool,
    link_url: String,
    in_code_block: bool,
    code_block_lang: Option<String>,
    code_block_content: String,
}

#[derive(Clone)]
struct ListContext {
    ordered: bool,
    index: usize,
}

impl<'a> TypstConverter<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            output: String::new(),
            list_stack: Vec::new(),
            in_table: false,
            table_alignments: Vec::new(),
            table_row: Vec::new(),
            current_cell: String::new(),
            in_heading: false,
            in_emphasis: false,
            in_strong: false,
            in_strikethrough: false,
            in_link: false,
            link_url: String::new(),
            in_code_block: false,
            code_block_lang: None,
            code_block_content: String::new(),
        }
    }

    fn convert(&mut self, events: Vec<Event<'_>>, frontmatter: &Frontmatter) -> String {
        // Add theme preamble
        self.output.push_str(&get_theme_preamble(
            &self.config.theme,
            self.config.paper_typst(),
        ));
        self.output.push('\n');

        // Add frontmatter header if present
        let header = frontmatter.to_typst_header();
        if !header.is_empty() {
            self.output.push_str(&header);
            self.output.push('\n');
        }

        for event in events {
            self.process_event(event);
        }

        self.output.clone()
    }

    fn process_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.text(&text),
            Event::Code(code) => self.inline_code(&code),
            Event::Html(html) => self.html(&html),
            Event::InlineHtml(html) => self.html(&html),
            Event::SoftBreak => self.soft_break(),
            Event::HardBreak => self.hard_break(),
            Event::Rule => self.rule(),
            Event::FootnoteReference(_) => {}
            Event::TaskListMarker(checked) => self.task_list_marker(checked),
            Event::InlineMath(math) => self.inline_math(&math),
            Event::DisplayMath(math) => self.display_math(&math),
        }
    }

    fn start_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => {
                if !self.in_table {
                    // Don't add extra newlines at the start
                    if !self.output.trim().is_empty() && !self.output.ends_with('\n') {
                        self.output.push('\n');
                    }
                }
            }
            Tag::Heading { level, .. } => {
                self.in_heading = true;
                let prefix = match level {
                    HeadingLevel::H1 => "= ",
                    HeadingLevel::H2 => "== ",
                    HeadingLevel::H3 => "=== ",
                    HeadingLevel::H4 => "==== ",
                    HeadingLevel::H5 => "===== ",
                    HeadingLevel::H6 => "====== ",
                };
                if !self.output.is_empty() {
                    self.output.push('\n');
                }
                self.output.push_str(prefix);
            }
            Tag::BlockQuote(_) => {
                self.output.push_str("\n#quote(block: true)[\n");
            }
            Tag::CodeBlock(kind) => {
                self.in_code_block = true;
                self.code_block_content.clear();
                self.code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.to_string()),
                    _ => None,
                };
            }
            Tag::List(start) => {
                let ordered = start.is_some();
                let index = start.unwrap_or(1) as usize;
                self.list_stack.push(ListContext { ordered, index });
                self.output.push('\n');
            }
            Tag::Item => {
                let indent = "  ".repeat(self.list_stack.len().saturating_sub(1));
                if let Some(ctx) = self.list_stack.last_mut() {
                    if ctx.ordered {
                        self.output.push_str(&format!("{}+ ", indent));
                        ctx.index += 1;
                    } else {
                        self.output.push_str(&format!("{}- ", indent));
                    }
                }
            }
            Tag::Emphasis => {
                self.in_emphasis = true;
                self.output.push('_');
            }
            Tag::Strong => {
                self.in_strong = true;
                self.output.push('*');
            }
            Tag::Strikethrough => {
                self.in_strikethrough = true;
                self.output.push_str("#strike[");
            }
            Tag::Link { dest_url, .. } => {
                self.in_link = true;
                self.link_url = dest_url.to_string();
            }
            Tag::Image { dest_url, .. } => {
                self.output
                    .push_str(&format!("#image(\"{}\")", escape_typst_string(&dest_url)));
            }
            Tag::Table(alignments) => {
                self.in_table = true;
                self.table_alignments = alignments;
                self.output.push_str("\n#table(\n  columns: (");
                let cols: Vec<&str> = self.table_alignments.iter().map(|_| "auto").collect();
                self.output.push_str(&cols.join(", "));
                self.output.push_str("),\n  align: (");
                let aligns: Vec<&str> = self
                    .table_alignments
                    .iter()
                    .map(|a| match a {
                        Alignment::Left => "left",
                        Alignment::Center => "center",
                        Alignment::Right => "right",
                        Alignment::None => "left",
                    })
                    .collect();
                self.output.push_str(&aligns.join(", "));
                self.output.push_str("),\n");
            }
            Tag::TableHead => {
                self.table_row.clear();
            }
            Tag::TableRow => {
                self.table_row.clear();
            }
            Tag::TableCell => {
                self.current_cell.clear();
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                if !self.in_table {
                    self.output.push_str("\n\n");
                }
            }
            TagEnd::Heading(_) => {
                self.in_heading = false;
                self.output.push_str("\n\n");
            }
            TagEnd::BlockQuote(_) => {
                self.output.push_str("]\n\n");
            }
            TagEnd::CodeBlock => {
                self.in_code_block = false;
                let lang = self.code_block_lang.take();
                let content = std::mem::take(&mut self.code_block_content);

                // Escape backticks in the content by using more backticks
                let backticks = "```";

                if let Some(lang) = lang {
                    self.output.push_str(&format!(
                        "\n{}{}\n{}\n{}\n\n",
                        backticks,
                        lang,
                        content.trim_end(),
                        backticks
                    ));
                } else {
                    self.output.push_str(&format!(
                        "\n{}\n{}\n{}\n\n",
                        backticks,
                        content.trim_end(),
                        backticks
                    ));
                }
            }
            TagEnd::List(_) => {
                self.list_stack.pop();
                if self.list_stack.is_empty() {
                    self.output.push('\n');
                }
            }
            TagEnd::Item => {
                if !self.output.ends_with('\n') {
                    self.output.push('\n');
                }
            }
            TagEnd::Emphasis => {
                self.in_emphasis = false;
                self.output.push('_');
            }
            TagEnd::Strong => {
                self.in_strong = false;
                self.output.push('*');
            }
            TagEnd::Strikethrough => {
                self.in_strikethrough = false;
                self.output.push(']');
            }
            TagEnd::Link => {
                self.in_link = false;
                // URL may have already been consumed by text() handler
                self.link_url.clear();
            }
            TagEnd::Table => {
                self.in_table = false;
                self.output.push_str(")\n\n");
            }
            TagEnd::TableHead => {
                // Output header row with bold
                for cell in &self.table_row {
                    self.output.push_str(&format!("  [*{}*],\n", cell));
                }
            }
            TagEnd::TableRow => {
                if !self.table_row.is_empty() {
                    for cell in &self.table_row {
                        self.output.push_str(&format!("  [{}],\n", cell));
                    }
                }
            }
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut self.current_cell);
                self.table_row.push(cell);
            }
            _ => {}
        }
    }

    fn text(&mut self, text: &str) {
        if self.in_code_block {
            self.code_block_content.push_str(text);
            return;
        }

        let escaped = escape_typst_text(text);

        if self.in_table {
            self.current_cell.push_str(&escaped);
        } else if self.in_link {
            // For links, we need to handle it differently
            self.output.push_str(&format!(
                "#link(\"{}\")[{}]",
                escape_typst_string(&self.link_url),
                escaped
            ));
            self.link_url.clear(); // Clear so end_tag doesn't duplicate
        } else {
            self.output.push_str(&escaped);
        }
    }

    fn inline_code(&mut self, code: &str) {
        if self.in_table {
            self.current_cell.push_str(&format!("`{}`", code));
        } else {
            self.output.push_str(&format!("`{}`", code));
        }
    }

    fn html(&mut self, _html: &str) {
        // HTML is not supported, skip it
    }

    fn soft_break(&mut self) {
        if self.in_code_block {
            self.code_block_content.push('\n');
        } else if !self.in_table {
            self.output.push(' ');
        }
    }

    fn hard_break(&mut self) {
        if self.in_code_block {
            self.code_block_content.push('\n');
        } else {
            self.output.push_str(" \\\n");
        }
    }

    fn rule(&mut self) {
        self.output.push_str("\n#line(length: 100%)\n\n");
    }

    fn task_list_marker(&mut self, checked: bool) {
        let marker = if checked { "[x]" } else { "[ ]" };
        self.output.push_str(marker);
        self.output.push(' ');
    }

    fn inline_math(&mut self, math: &str) {
        self.output.push_str(&format!("${}$", math));
    }

    fn display_math(&mut self, math: &str) {
        self.output.push_str(&format!("\n$ {} $\n", math));
    }
}

fn escape_typst_text(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

fn escape_typst_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::markdown::parse_markdown;

    fn convert_md(md: &str) -> String {
        let config = Config::default();
        let events = parse_markdown(md);
        let fm = Frontmatter::default();
        to_typst(events, &fm, &config)
    }

    #[test]
    fn test_heading() {
        let result = convert_md("# Hello");
        assert!(result.contains("= Hello"));
    }

    #[test]
    fn test_paragraph() {
        let result = convert_md("Hello world");
        assert!(result.contains("Hello world"));
    }

    #[test]
    fn test_emphasis() {
        let result = convert_md("*italic*");
        assert!(result.contains("_italic_"));
    }

    #[test]
    fn test_strong() {
        let result = convert_md("**bold**");
        assert!(result.contains("*bold*"));
    }

    #[test]
    fn test_code_inline() {
        let result = convert_md("`code`");
        assert!(result.contains("`code`"));
    }

    #[test]
    fn test_list() {
        let result = convert_md("- item 1\n- item 2");
        assert!(result.contains("- item 1"));
        assert!(result.contains("- item 2"));
    }
}
