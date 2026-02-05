mod close;
mod content;
mod description;
mod dialog;
mod footer;
mod header;
mod title;

pub use close::*;
pub use content::*;
pub use description::*;
pub use dialog::*;
pub use footer::*;
pub use header::*;
pub use title::*;

use crate::Reactive;
use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct DialogContext {
    pub alert: Reactive<bool>,
    pub described_by: RwSignal<String>,
    pub labelled_by: RwSignal<String>,
    pub open: Reactive<bool>,
}
