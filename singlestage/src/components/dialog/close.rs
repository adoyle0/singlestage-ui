use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct DialogCloseContext {}

/// Contains the dialog title and a description to be rendered in the open dialog.
#[component]
pub fn DialogClose(children: Children) -> impl IntoView {
    view! { <Provider value=DialogCloseContext {}>{children()}</Provider> }
}
