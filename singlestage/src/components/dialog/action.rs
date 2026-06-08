use crate::primitives::*;
use leptos::prelude::*;

/// Wraps a Button that the user presses to acknowledge the dialog.
#[component]
pub fn DialogAction(children: Children) -> impl IntoView {
    view! { <DialogActionPrimitive>{children()}</DialogActionPrimitive> }
}
