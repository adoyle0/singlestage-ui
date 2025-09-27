use crate::DropdownMenuContext;
use leptos::prelude::*;

/// Contains all the parts of a dropdown menu.
#[component]
pub fn DropdownMenu(
    children: Children,
    /// Set classes for the dropdown menu container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let menu_id = RwSignal::new(String::new());
    let trigger_id = RwSignal::new(String::new());

    let context = DropdownMenuContext {
        menu_id,
        trigger_id,
    };
    provide_context(context);

    view! {
        <div class=move || {
            format!("singlestage-dropdown-menu {}", class.get().unwrap_or_default())
        }>{children()}</div>
    }
}
