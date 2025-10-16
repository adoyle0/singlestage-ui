mod collapsible;
mod content;
mod footer;
mod group;
mod header;
mod menu;
mod provider;
mod separator;
mod sidebar;
mod trigger;

pub use collapsible::*;
pub use content::*;
pub use footer::*;
pub use group::*;
pub use header::*;
pub use menu::*;
pub use provider::*;
pub use separator::*;
pub use sidebar::*;
pub use trigger::*;

use leptos::{ev::MouseEvent, prelude::*};
use std::sync::Arc;

// Large/small screen breakpoint magic numbers for show/hide
const BREAKPOINT_REM: usize = 48;
const BREAKPOINT_PX: usize = 768;
const DEFAULT_FONT_PX: usize = 16;
const SIDEBAR_MOBILE_WIDTH_REM: usize = 18; // main.css

#[derive(Clone)]
pub struct SidebarContext {
    close_if_small_screen: Arc<dyn Fn() + Send + Sync>,
    pub hidden: RwSignal<bool>,
    pub side: RwSignal<String>,
}

impl SidebarContext {
    pub fn close_if_small_screen(&self) {
        (self.close_if_small_screen)()
    }
}

/// Get current viewport width
fn get_screen_width() -> Option<f64> {
    if let Ok(js_width) = window().inner_width()
        && let Some(f64_width) = js_width.as_f64() {
            return Some(f64_width);
        }

    None
}

/// Get the font size setting of the browser viewing the page
fn get_font_size() -> Option<usize> {
    if let Some(dom) = document().document_element()
        && let Ok(Some(css)) = window().get_computed_style(&dom)
            && let Ok(mut font_size) = css.get_property_value("font-size") {
                // Cut off "px"
                let _ = font_size.split_off(font_size.len() - 2);

                if let Ok(parsed_font_size) = font_size.parse::<usize>() {
                    return Some(parsed_font_size);
                }
            }

    None
}

/// Determine if the viewport is smaller than 48rem wide
fn screen_is_small() -> bool {
    let breakpoint;
    if let Some(font_size) = get_font_size() {
        breakpoint = font_size * BREAKPOINT_REM;
    } else {
        breakpoint = BREAKPOINT_PX;
    }

    if let Some(screen_width) = get_screen_width()
        && screen_width < breakpoint as f64 {
            return true;
        }

    false
}
