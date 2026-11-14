mod content;
mod description;
mod error;
mod field;
mod group;
mod legend;
mod set;
mod title;

pub use content::*;
pub use description::*;
pub use error::*;
pub use field::*;
pub use group::*;
pub use legend::*;
pub use set::*;
pub use title::*;

#[cfg(feature = "checkbox")]
mod checkbox_group;
#[cfg(feature = "checkbox")]
pub use checkbox_group::*;

#[cfg(feature = "label")]
mod label;
#[cfg(feature = "label")]
pub use label::*;

#[cfg(feature = "radio")]
mod radio_group;
#[cfg(feature = "radio")]
pub use radio_group::*;

#[cfg(feature = "separator")]
mod separator;
#[cfg(feature = "separator")]
pub use separator::*;

use crate::Reactive;
use leptos::prelude::RwSignal;

#[derive(Clone)]
pub(crate) struct FieldContext {
    pub description_id: RwSignal<String>,
    pub disabled: Reactive<bool>,
    pub has_title: RwSignal<bool>,
    pub input_id: RwSignal<String>,
    pub invalid: Reactive<bool>,
    pub label_id: RwSignal<String>,
    pub required: Reactive<bool>,
}

#[derive(Clone)]
pub(crate) struct FieldSetContext {}
