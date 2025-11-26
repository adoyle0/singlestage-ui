#![allow(clippy::module_inception)]
use std::include_str;
mod patch_class;
mod as_child;
pub use as_child::AsChild;
mod components;
pub mod reactive;
pub use components::*;
pub use reactive::*;

#[cfg(feature = "macro")]
extern crate singlestage_macro;
#[cfg(feature = "macro")]
pub use singlestage_macro::*;

#[cfg(feature = "theme_provider")]
static CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage.css"));
