// TODO: Work on this

use leptos::prelude::*;

#[component]
pub fn SidebarInset(children: Children) -> impl IntoView {
    view! { <main class="singlestage-sidebar-inset">{children()}</main> }
}
