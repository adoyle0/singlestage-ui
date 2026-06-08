use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct SheetCloseContext {}

#[component]
pub fn SheetClose(children: Children) -> impl IntoView {
    view! { <Provider value=SheetCloseContext {}>{children()}</Provider> }
}
