#![allow(clippy::module_inception)]
#![feature(stmt_expr_attributes)]

mod components;
pub mod reactive;

pub use components::*;
pub use reactive::*;
pub(crate) mod primitives;

#[cfg(feature = "macro")]
extern crate singlestage_macro;
#[cfg(feature = "macro")]
pub use singlestage_macro::*;

use std::include_str;

#[cfg(feature = "theme_provider")]
static CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage.css"));
#[cfg(feature = "theme_provider")]
static CSS_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_dark.css"));
