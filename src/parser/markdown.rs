use pulldown_cmark::{Event, Options, Parser};

/// Parse markdown content and return an iterator of events
pub fn parse_markdown(content: &str) -> Vec<Event<'_>> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_MATH;

    Parser::new_ext(content, options).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{Event, Tag};

    #[test]
    fn test_parse_heading() {
        let events = parse_markdown("# Hello");
        assert!(matches!(events[0], Event::Start(Tag::Heading { .. })));
    }

    #[test]
    fn test_parse_paragraph() {
        let events = parse_markdown("Hello world");
        assert!(matches!(events[0], Event::Start(Tag::Paragraph)));
        assert!(matches!(events[1], Event::Text(_)));
    }

    #[test]
    fn test_parse_code_block() {
        let events = parse_markdown("```rust\nlet x = 1;\n```");
        let has_code_block = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::CodeBlock(_))));
        assert!(has_code_block);
    }

    #[test]
    fn test_parse_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let events = parse_markdown(md);
        let has_table = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::Table(_))));
        assert!(has_table);
    }

    #[test]
    fn test_parse_list() {
        let events = parse_markdown("- item 1\n- item 2");
        let has_list = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::List(_))));
        assert!(has_list);
    }
}
