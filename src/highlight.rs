extern crate syntect;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::easy::HighlightLines;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};

/// Takes the content of a paste and the extension passed in by the viewer and will return the content
/// highlighted in the appropriate format in HTML.
pub fn highlight(content: &str, ext: &str) -> Option<String> {
    lazy_static! {
        static ref SS: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref TS: ThemeSet = ThemeSet::load_defaults();
    }

    let syntax = SS.find_syntax_by_extension(ext)?;
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);
    let regions = h.highlight(content, &SS);

    Some(styled_line_to_highlighted_html(&regions[..], IncludeBackground::No))
}