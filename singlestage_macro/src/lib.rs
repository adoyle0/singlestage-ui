use proc_macro::TokenStream;

mod icon;

/// The icon macro for working with icondata icons.
/// It accepts optional named arguments and generates a view containing a SVG element.
#[proc_macro]
pub fn icon(input: TokenStream) -> TokenStream {
    icon::icon(input)
}
