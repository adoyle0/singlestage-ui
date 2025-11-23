mod content;
mod description;
mod error;
mod field;
mod group;
mod label;
mod legend;
mod separator;
mod set;
mod title;

pub use content::*;
pub use description::*;
pub use error::*;
pub use field::*;
pub use group::*;
pub use label::*;
pub use legend::*;
pub use separator::*;
pub use set::*;
pub use title::*;

use leptos::prelude::RwSignal;

#[derive(Clone)]
pub struct FieldContext {
    pub description_id: RwSignal<String>,
    pub input_id: RwSignal<String>,
    pub label_id: RwSignal<String>,
}
