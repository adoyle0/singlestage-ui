use crate::SidebarContext;
use leptos::prelude::*;

/// A clickable rail on the inner edge of the sidebar that works as an open button.
#[component]
pub fn SidebarRail() -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <button
            class="singlestage-sidebar-rail"
            on:click=move |_| sidebar.open.set(!sidebar.open.get_untracked())
        />
    }
}
