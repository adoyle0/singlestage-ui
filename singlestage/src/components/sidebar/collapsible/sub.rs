use leptos::prelude::*;

/// Wraps a submenu within a SidebarMenu.
#[component]
pub fn SidebarMenuSub(children: Children) -> impl IntoView {
    view! { {children()} }
}
