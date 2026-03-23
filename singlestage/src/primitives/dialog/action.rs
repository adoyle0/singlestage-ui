use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct DialogActionContext {}

#[component]
pub fn DialogActionPrimitive(children: Children) -> impl IntoView {
    view! { <Provider value=DialogActionContext {}>{children()}</Provider> }
}
