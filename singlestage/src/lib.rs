#![allow(clippy::module_inception)]
use std::include_str;
mod patch_class;
mod as_child;
pub use as_child::AsChild;
mod components;
pub mod reactive;
pub use components::*;
pub use reactive::*;

extern crate singlestage_macro;
pub use singlestage_macro::*;

static CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage.css"));
