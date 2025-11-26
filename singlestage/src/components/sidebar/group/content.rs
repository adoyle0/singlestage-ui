use leptos::prelude::*;

/// Wraps the content of a sidebar group.
#[component]
pub fn SidebarGroupContent(children: Children) -> impl IntoView {
    view! { {children()} }
}
