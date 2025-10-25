extern crate proc_macro;

use proc_macro::TokenStream;

mod svg;

/// The main svg procedural macro function.
/// It accepts optional named arguments
/// and generates an SVG string.
#[proc_macro]
pub fn svg(input: TokenStream) -> TokenStream {
    svg::svg(input)
}
