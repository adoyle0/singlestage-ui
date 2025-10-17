use crate::Reactive;

mod content;
mod list;
mod tabs;
mod trigger;

pub use content::*;
pub use list::*;
pub use tabs::*;
pub use trigger::*;

#[derive(Clone)]
pub struct TabsContext {
    value: Reactive<String>,
}
