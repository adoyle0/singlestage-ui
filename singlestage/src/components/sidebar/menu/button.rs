// TODO: make generic over button primitive with as_child

use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct SidebarMenuButtonContext {}

#[component]
pub fn SidebarMenuButton(children: Children) -> impl IntoView {
    view! { <Provider value=SidebarMenuButtonContext {}>{children()}</Provider> }
}
