mod action;
mod cancel;
mod content;
mod description;
mod dialog;
mod footer;
mod header;
mod media;
mod title;

pub use action::*;
pub use cancel::*;
pub use content::*;
pub use description::*;
pub use dialog::*;
pub use footer::*;
pub use header::*;
pub use media::*;
pub use title::*;

use crate::Reactive;

#[derive(Clone, Default)]
pub(crate) struct DialogContext {
    pub alert: bool,
    pub open: Reactive<bool>,
}
