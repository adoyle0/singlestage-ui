use crate::primitives::*;
use leptos::prelude::*;

/// A context provider that wraps a Button that the user presses to acknowledge the dialog.
#[component]
pub fn DialogAction(children: Children) -> impl IntoView {
    view! { <DialogActionPrimitive>{children()}</DialogActionPrimitive> }
}
