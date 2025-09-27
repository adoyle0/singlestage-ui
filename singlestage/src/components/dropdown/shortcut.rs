use leptos::prelude::*;

/// Displays next to the item content and indicates keyboard shortcuts.
#[component]
pub fn DropdownMenuShortcut(children: Children) -> impl IntoView {
    view! { <span class="singlestage-dropdown-menu-shortcut">{children()}</span> }
}
