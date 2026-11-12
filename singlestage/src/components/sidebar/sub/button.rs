use crate::SidebarMenuButton;
use leptos::prelude::*;

/// Wraps content displayed in the click area of a SidebarMenuSubItem.
#[component]
pub fn SidebarMenuSubButton(
    children: Children,
    #[prop(optional, into)] tooltip: MaybeProp<String>,
) -> impl IntoView {
    view! { <SidebarMenuButton tooltip>{children()}</SidebarMenuButton> }
}
