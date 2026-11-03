use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct SheetCloseContext {}

/// Wraps a button to be used to trigger closing the sheet.
#[component]
pub fn SheetClose(children: Children) -> impl IntoView {
    view! { <Provider value=SheetCloseContext {}>{children()}</Provider> }
}
