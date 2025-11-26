use leptos::prelude::*;

/// Wraps content displayed in the click area of a SidebarMenuItem.
#[component]
pub fn SidebarMenuButton(children: Children) -> impl IntoView {
    view! { {children()} }
}
