mod close;
mod content;
mod description;
mod dialog;
mod footer;
mod header;
mod title;
mod trigger;

pub use close::*;
pub use content::*;
pub use description::*;
pub use dialog::*;
pub use footer::*;
pub use header::*;
pub use title::*;
pub use trigger::*;

use leptos::prelude::*;

#[derive(Clone)]
pub struct DialogContext {
    pub labeled_by: RwSignal<String>,
    pub described_by: RwSignal<String>,
}
