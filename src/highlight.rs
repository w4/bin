use bat::assets::HighlightingAssets;
use once_cell::sync::Lazy;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
};

thread_local!(pub static BAT_ASSETS: HighlightingAssets = HighlightingAssets::from_binary());

/// Takes the content of a paste and the extension passed in by the viewer and will return the content
/// highlighted in the appropriate format in HTML.
///
/// Returns `None` if the extension isn't supported.
pub fn highlight(content: &str, ext: &str) -> Option<String> {
    static SS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

    BAT_ASSETS.with(|f| {
        let ss = f.get_syntax_set().ok().unwrap_or(&SS);
        let syntax = ss.find_syntax_by_extension(ext)?;
        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, ss, ClassStyle::Spaced);
        for line in LinesWithEndings(content.trim()) {
            html_generator.parse_html_for_line_which_includes_newline(line);
        }
        Some(html_generator.finalize())
    })
}

pub struct LinesWithEndings<'a>(&'a str);

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            let split = self.0.find('\n').map_or(self.0.len(), |i| i + 1);
            let (line, rest) = self.0.split_at(split);
            self.0 = rest;
            Some(line)
        }
    }
}
