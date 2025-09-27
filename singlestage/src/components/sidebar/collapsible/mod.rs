mod collapsible;
mod content;
mod sub;
mod sub_item;

pub use collapsible::*;
pub use content::*;
pub use sub::*;
pub use sub_item::*;

#[derive(Clone)]
pub struct CollapsibleContext {
    id: String,
}
