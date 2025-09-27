use leptos::prelude::*;

/// Visually separates groups of menu items.
#[component]
pub fn DropdownMenuSeparator() -> impl IntoView {
    view! { <hr class="singlestage-dropdown-menu-separator" role="separator" /> }
}
