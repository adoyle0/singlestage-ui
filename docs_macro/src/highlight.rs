use std::sync::Arc;
use std::{io::BufRead, path::Path};
use syntect::highlighting::Color;
use syntect::{
    easy::HighlightFile,
    highlighting::Theme,
    html::{IncludeBackground, append_highlighted_html_for_styled_line},
    parsing::SyntaxSet,
};

pub fn highlight_html_from_file<P: AsRef<Path>>(
    path: P,
    ss: Arc<SyntaxSet>,
    theme: Arc<&Theme>,
    bg: Arc<Color>,
) -> String {
    let mut highlighter = HighlightFile::new(&path, &ss, &theme).unwrap();
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
                IncludeBackground::IfDifferent(*bg),
                &mut output,
            )
            .unwrap();
        } // until NLL this scope is needed so we can clear the buffer after
        line.clear(); // read_line appends so we need to clear between lines
    }
    output.push_str("</pre>\n");
    output
}
