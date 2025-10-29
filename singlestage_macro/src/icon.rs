use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Path, Result, Token, parse_macro_input};

struct MacroArgs {
    icon: Option<Path>,
    style: Option<Expr>,
    width: Option<Expr>,
    height: Option<Expr>,
    view_box: Option<Expr>,
    stroke_linecap: Option<Expr>,
    stroke_linejoin: Option<Expr>,
    stroke_width: Option<Expr>,
    stroke: Option<Expr>,
    fill: Option<Expr>,
    class: Option<Expr>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = MacroArgs {
            icon: None,
            style: None,
            width: None,
            height: None,
            view_box: None,
            stroke_linecap: None,
            stroke_linejoin: None,
            stroke_width: None,
            stroke: None,
            fill: None,
            class: None,
        };

        if !input.is_empty() {
            let value: Path = input.parse::<Path>()?;
            args.icon = Some(value);

            // Consume trailing comma if present
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        } else {
            return Err(input.error("The argument of type icondata::IconData) is required"));
        }

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _colon: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;

            let key_str = key.to_string();

            match key_str.as_str() {
                "style" => args.style = Some(value),
                "width" => args.width = Some(value),
                "height" => args.height = Some(value),
                "view_box" => args.view_box = Some(value),
                "stroke_linecap" => args.stroke_linecap = Some(value),
                "stroke_linejoin" => args.stroke_linejoin = Some(value),
                "stroke_width" => args.stroke_width = Some(value),
                "stroke" => args.stroke = Some(value),
                "fill" => args.fill = Some(value),
                "class" => args.class = Some(value),
                _ => return Err(input.error(format!("unrecognized argument name: {}", key_str))),
            }

            // Consume trailing comma if present
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(args)
    }
}

/// The icon macro for working with icondata icons.
/// It accepts optional named arguments and generates a view containing a SVG element.
pub fn icon(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as MacroArgs);

    // If we arrive here, then args.icon is guaranteed to be valid
    let icon_expr = args.icon.expect("Failure reading icon");

    // Assign provided expressions or default literal values for optional arguments
    let style = match args.style {
        Some(s) => s.to_token_stream(),
        None => quote! {#icon_expr.style.unwrap_or_default()},
    };

    let width = match args.width {
        Some(w) => w.to_token_stream(),
        None => quote! {#icon_expr.width.unwrap_or("100%")},
    };

    let height = match args.height {
        Some(h) => h.to_token_stream(),
        None => quote! {#icon_expr.height.unwrap_or("100%")},
    };

    let view_box = match args.view_box {
        Some(v) => v.to_token_stream(),
        None => quote! {#icon_expr.view_box.unwrap_or_default()},
    };

    let stroke_linecap = match args.stroke_linecap {
        Some(s) => s.to_token_stream(),
        None => quote! {#icon_expr.stroke_linecap.unwrap_or("butt")},
    };

    let stroke_linejoin = match args.stroke_linejoin {
        Some(s) => s.to_token_stream(),
        None => quote! {#icon_expr.stroke_linejoin.unwrap_or("miter")},
    };

    let stroke_width = match args.stroke_width {
        Some(sw) => sw.to_token_stream(),
        None => quote! {#icon_expr.stroke_width.unwrap_or("1px")},
    };

    let stroke = match args.stroke {
        Some(s) => s.to_token_stream(),
        None => quote! {#icon_expr.stroke.unwrap_or("currentColor")},
    };

    let fill = match args.fill {
        Some(h) => h.to_token_stream(),
        None => quote! {#icon_expr.fill.unwrap_or("currentColor")},
    };

    let class = match args.class {
        Some(c) => c.to_token_stream(),
        None => quote! {"".to_string()},
    };

    let expanded = quote! {
        view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                style=#style
                width=#width
                height=#height
                viewBox=#view_box
                stroke-linecap=#stroke_linecap
                stroke-linejoin=#stroke_linejoin
                stroke-width=#stroke_width
                stroke=#stroke
                fill=#fill
                class=#class
                inner_html=#icon_expr.data
            ></svg>
        }
    };

    expanded.into()
}
