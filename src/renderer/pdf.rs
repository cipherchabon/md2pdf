use crate::config::Config;
use crate::{Md2PdfError, Result};
use std::collections::HashMap;
use std::sync::OnceLock;
use typst::diag::{FileError, FileResult, Severity};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt};

static FONTS: OnceLock<(LazyHash<FontBook>, Vec<Font>)> = OnceLock::new();

pub fn render_pdf(typst_code: &str, _config: &Config) -> Result<Vec<u8>> {
    let world = Md2PdfWorld::new(typst_code.to_string());

    let result = typst::compile(&world);

    match result.output {
        Ok(doc) => {
            let options = typst_pdf::PdfOptions::default();
            match typst_pdf::pdf(&doc, &options) {
                Ok(pdf) => Ok(pdf),
                Err(errors) => {
                    let error_messages: Vec<String> = errors
                        .iter()
                        .map(|e| format!("{:?}", e))
                        .collect();
                    Err(Md2PdfError::Typst(error_messages.join("\n")))
                }
            }
        }
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .iter()
                .map(|e| {
                    let severity = match e.severity {
                        Severity::Error => "error",
                        Severity::Warning => "warning",
                    };
                    format!("{}: {}", severity, e.message)
                })
                .collect();
            Err(Md2PdfError::Typst(error_messages.join("\n")))
        }
    }
}

struct Md2PdfWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: Source,
    files: HashMap<FileId, Source>,
}

impl Md2PdfWorld {
    fn new(source: String) -> Self {
        let (book, fonts) = FONTS
            .get_or_init(|| {
                let mut book = FontBook::new();
                let mut fonts = Vec::new();

                // Load embedded fonts from typst-assets
                for data in typst_assets::fonts() {
                    let buffer = Bytes::new(data);
                    for font in Font::iter(buffer) {
                        book.push(font.info().clone());
                        fonts.push(font);
                    }
                }

                // Load system fonts as additional options
                #[cfg(target_os = "macos")]
                {
                    for entry in std::fs::read_dir("/System/Library/Fonts").into_iter().flatten() {
                        if let Ok(entry) = entry {
                            Self::load_font_file(&entry.path(), &mut book, &mut fonts);
                        }
                    }

                    if let Some(home) = std::env::var_os("HOME") {
                        let user_fonts = std::path::Path::new(&home).join("Library/Fonts");
                        for entry in std::fs::read_dir(user_fonts).into_iter().flatten() {
                            if let Ok(entry) = entry {
                                Self::load_font_file(&entry.path(), &mut book, &mut fonts);
                            }
                        }
                    }
                }

                #[cfg(target_os = "linux")]
                {
                    for dir in ["/usr/share/fonts", "/usr/local/share/fonts"] {
                        Self::load_fonts_recursive(std::path::Path::new(dir), &mut book, &mut fonts);
                    }

                    if let Some(home) = std::env::var_os("HOME") {
                        let user_fonts = std::path::Path::new(&home).join(".fonts");
                        Self::load_fonts_recursive(&user_fonts, &mut book, &mut fonts);
                    }
                }

                (LazyHash::new(book), fonts)
            })
            .clone();

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let main = Source::new(main_id, source);

        Self {
            library: LazyHash::new(Library::default()),
            book,
            fonts,
            main,
            files: HashMap::new(),
        }
    }

    fn load_font_file(path: &std::path::Path, book: &mut FontBook, fonts: &mut Vec<Font>) {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !matches!(ext.to_lowercase().as_str(), "ttf" | "otf" | "ttc" | "otc") {
            return;
        }

        if let Ok(data) = std::fs::read(path) {
            let buffer = Bytes::new(data);
            for font in Font::iter(buffer) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn load_fonts_recursive(path: &std::path::Path, book: &mut FontBook, fonts: &mut Vec<Font>) {
        if !path.exists() {
            return;
        }

        for entry in std::fs::read_dir(path).into_iter().flatten() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    Self::load_fonts_recursive(&path, book, fonts);
                } else {
                    Self::load_font_file(&path, book, fonts);
                }
            }
        }
    }
}

impl typst::World for Md2PdfWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else if let Some(source) = self.files.get(&id) {
            Ok(source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let now = chrono::Local::now();
        Datetime::from_ymd(
            now.format("%Y").to_string().parse().ok()?,
            now.format("%m").to_string().parse().ok()?,
            now.format("%d").to_string().parse().ok()?,
        )
    }
}
