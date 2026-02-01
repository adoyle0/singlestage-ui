mod avatar;
mod badge;
mod fallback;
mod group;
mod group_count;
mod image;

pub use avatar::*;
pub use badge::*;
pub use fallback::*;
pub use group::*;
pub use group_count::*;
pub use image::*;

use leptos::prelude::*;

#[derive(Clone)]
struct AvatarContext {
    img_loaded: RwSignal<bool>,
}
