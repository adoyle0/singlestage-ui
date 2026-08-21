use crate::SidebarContext;
use leptos::prelude::*;

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
