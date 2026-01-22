use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub abstract_text: Option<String>,
}

impl Frontmatter {
    /// Extract frontmatter from markdown content
    /// Returns (frontmatter, remaining_content)
    pub fn extract(content: &str) -> Result<(Self, &str)> {
        let content = content.trim_start();

        if !content.starts_with("---") {
            return Ok((Self::default(), content));
        }

        let after_first_delimiter = &content[3..];
        let Some(end_pos) = after_first_delimiter.find("\n---") else {
            return Ok((Self::default(), content));
        };

        let yaml_content = &after_first_delimiter[..end_pos].trim();
        let remaining = &after_first_delimiter[end_pos + 4..].trim_start();

        let frontmatter: Frontmatter = serde_yaml::from_str(yaml_content)?;
        Ok((frontmatter, remaining))
    }

    pub fn to_typst_header(&self) -> String {
        let mut parts = Vec::new();

        if let Some(title) = &self.title {
            parts.push(format!(
                r#"#align(center, text(size: 24pt, weight: "bold")[{}])"#,
                escape_typst(title)
            ));
        }

        if let Some(author) = &self.author {
            parts.push(format!(
                r#"#align(center, text(size: 12pt)[{}])"#,
                escape_typst(author)
            ));
        }

        if let Some(date) = &self.date {
            parts.push(format!(
                r#"#align(center, text(size: 11pt, style: "italic")[{}])"#,
                escape_typst(date)
            ));
        }

        if !parts.is_empty() {
            parts.push(String::new()); // Add blank line after header
        }

        parts.join("\n\n")
    }
}

fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_frontmatter() {
        let content = "# Hello World\n\nSome text";
        let (fm, remaining) = Frontmatter::extract(content).unwrap();
        assert!(fm.title.is_none());
        assert_eq!(remaining, content);
    }

    #[test]
    fn test_with_frontmatter() {
        let content = r#"---
title: My Document
author: John Doe
date: 2025-01-21
---

# Hello World
"#;
        let (fm, remaining) = Frontmatter::extract(content).unwrap();
        assert_eq!(fm.title, Some("My Document".to_string()));
        assert_eq!(fm.author, Some("John Doe".to_string()));
        assert_eq!(fm.date, Some("2025-01-21".to_string()));
        assert!(remaining.starts_with("# Hello"));
    }
}
