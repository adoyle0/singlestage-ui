#![allow(clippy::module_inception)]
use std::include_str;
mod components;
pub use components::*;

static CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage.css"));

/// Helper macro for using icondata icons
#[macro_export]
macro_rules! icon {
    ( $icon:expr ) => {{
        view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                style=$icon.style.unwrap_or_default()
                width=$icon.width.unwrap_or("24")
                height=$icon.height.unwrap_or("24")
                viewBox=$icon.view_box.unwrap_or_default()
                stroke-linecap=$icon.stroke_linecap.unwrap_or_default()
                stroke-linejoin=$icon.stroke_linejoin.unwrap_or_default()
                stroke-width=$icon.stroke_width.unwrap_or_default()
                stroke=$icon.stroke.unwrap_or("currentColor")
                fill=$icon.fill.unwrap_or("currentColor")
                inner_html=$icon.data
            ></svg>
        }
    }};
}
