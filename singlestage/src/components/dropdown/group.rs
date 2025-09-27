use crate::DropdownMenuGroupContext;
use leptos::prelude::*;

/// Contains multiple items.
#[component]
pub fn DropdownMenuGroup(children: Children) -> impl IntoView {
    let heading_id = RwSignal::new(String::new());

    let context = DropdownMenuGroupContext { heading_id };
    provide_context(context);

    view! {
        <div
            role="group"
            class="singlestage-dropdown-menu-group"
            aria-labelledby=move || heading_id.get()
        >
            {children()}
        </div>
    }
}
