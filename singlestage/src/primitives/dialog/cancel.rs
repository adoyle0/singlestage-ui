use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct DialogCancelContext {}

#[component]
pub fn DialogCancelPrimitive(children: Children) -> impl IntoView {
    view! { <Provider value=DialogCancelContext {}>{children()}</Provider> }
}
