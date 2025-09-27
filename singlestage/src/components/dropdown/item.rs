use leptos::prelude::*;

/// Contains a menu item.
#[component]
pub fn DropdownMenuItem(
    children: Children,
    /// Controls whether the item appears disabled and is clickable.
    #[prop(optional, into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <li
            role="menuitem"
            class="singlestage-dropdown-menu-item"
            aria-disabled=move || disabled.get().to_string()
        >
            {children()}
        </li>
    }
}
