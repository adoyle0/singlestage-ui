use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct BreadcrumbLinkContext {}

#[component]
pub fn BreadcrumbLink(children: Children) -> impl IntoView {
    view! { <Provider value=BreadcrumbLinkContext {}>{children()}</Provider> }
}
