use std::path::Path;

/// Check if a path is a local image
pub fn is_local_image(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp"
    )
}

/// Check if a URL is a remote image
pub fn is_remote_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_image() {
        assert!(is_local_image("image.png"));
        assert!(is_local_image("path/to/image.jpg"));
        assert!(!is_local_image("document.pdf"));
    }

    #[test]
    fn test_remote_url() {
        assert!(is_remote_url("https://example.com/image.png"));
        assert!(is_remote_url("http://example.com/image.png"));
        assert!(!is_remote_url("/local/path/image.png"));
    }
}
