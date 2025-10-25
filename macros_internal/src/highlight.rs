use std::{io::BufRead, path::Path};
use syntect::{
    easy::HighlightFile,
    highlighting::ThemeSet,
    html::{
        IncludeBackground, append_highlighted_html_for_styled_line, start_highlighted_html_snippet,
    },
    parsing::SyntaxSet,
};

pub fn highlight_html_from_file<P: AsRef<Path>>(path: P) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["Solarized (light)"];

    let mut highlighter = HighlightFile::new(&path, &ss, theme).expect("Error highlighting file");
    let (_, bg) = start_highlighted_html_snippet(theme);
    let mut output = "<pre>".to_string();

    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap() > 0 {
        {
            let regions = highlighter
                .highlight_lines
                .highlight_line(&line, &ss)
                .unwrap();
            append_highlighted_html_for_styled_line(
                &regions[..],
                IncludeBackground::IfDifferent(bg),
                &mut output,
            )
            .unwrap();
        }
        line.clear();
    }
    output.push_str("</pre>\n");
    output
}
