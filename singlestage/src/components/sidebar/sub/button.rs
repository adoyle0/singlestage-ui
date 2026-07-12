use leptos::{context::Provider, prelude::*};

use crate::SidebarMenuButtonContext;

#[component]
pub fn SidebarMenuSubButton(children: Children) -> impl IntoView {
    view! { <Provider value=SidebarMenuButtonContext {}>{children()}</Provider> }
}
