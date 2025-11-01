use proc_macro::TokenStream;

mod icon;

/// The icon macro for working with icondata icons.
/// It accepts optional named arguments and generates a Leptos `view!` containing a SVG element.
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
    icon::icon(input)
}
