// NOTE: This is intentionally a context provider and not another `Link` component
// to be more flexible with its children (`a` vs `A` vs `Link`, etc.)
// at the cost of being slightly more verbose than shadcn

use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct BreadcrumbLinkContext {}

#[component]
pub fn BreadcrumbLink(children: Children) -> impl IntoView {
    view! { <Provider value=BreadcrumbLinkContext {}>{children()}</Provider> }
}
