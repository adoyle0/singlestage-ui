use proc_macro::TokenStream;

mod svg;

/// Generates an `<svg>` tag wrapped in a leptos `view!`
///
/// # Arguments
///
/// The first parameter must be an icondata id. Some examples may be found here : [Carloskiki's Icondata](https://carloskiki.github.io/icondata/)
///
/// Notice the underscores.
///
/// * `stroke_linecap`
/// * `stroke_linejoin`
/// * `stroke_width`  
/// * `width`
/// * `height`
/// * `stroke`
/// * `fill`
/// * `style`
/// * `class`
///
/// # Examples
///
/// ```rust
/// use singlestage::icon;
///
///<Label>
///  {icon!(icondata::FiCreditCard)} "Card"
/// </Label>
///
///<Label>
///  {icon!(icondata::FiCreditCard, width=100, class="mr-3")} "Card"
/// </Label>
/// ```
#[proc_macro]
pub fn icon(input: TokenStream) -> TokenStream {
    svg::svg(input)
}
