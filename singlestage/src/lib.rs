#![allow(clippy::module_inception)]

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

#[cfg(feature = "style_luma")]
static CSS_LUMA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_luma.css"));
#[cfg(feature = "style_luma")]
static CSS_LUMA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_luma_dark.css"));

#[cfg(feature = "style_lyra")]
static CSS_LYRA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_lyra.css"));
#[cfg(feature = "style_lyra")]
static CSS_LYRA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_lyra_dark.css"));

#[cfg(feature = "style_maia")]
static CSS_MAIA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_maia.css"));
#[cfg(feature = "style_maia")]
static CSS_MAIA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_maia_dark.css"));

#[cfg(feature = "style_mira")]
static CSS_MIRA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_mira.css"));
#[cfg(feature = "style_mira")]
static CSS_MIRA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_mira_dark.css"));

#[cfg(feature = "style_nova")]
static CSS_NOVA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_nova.css"));
#[cfg(feature = "style_nova")]
static CSS_NOVA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_nova_dark.css"));

#[cfg(feature = "style_sera")]
static CSS_SERA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_sera.css"));
#[cfg(feature = "style_sera")]
static CSS_SERA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_sera_dark.css"));

#[cfg(feature = "style_vega")]
static CSS_VEGA: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_vega.css"));
#[cfg(feature = "style_vega")]
static CSS_VEGA_DARK: &str = include_str!(concat!(env!("OUT_DIR"), "/singlestage_vega_dark.css"));
