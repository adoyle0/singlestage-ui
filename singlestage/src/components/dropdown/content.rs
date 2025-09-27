use crate::DropdownMenuContext;
use leptos::prelude::*;

/// The component that pops out when the dropdown menu is open.
#[component]
pub fn DropdownMenuContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let menu = expect_context::<DropdownMenuContext>();

    let uuid = uuid::Uuid::new_v4().to_string();
    menu.menu_id.set(uuid.clone());

    view! {
        <div class="singlestage-dropdown-menu-content" id=uuid popover="auto">
            <menu role="menu" class=move || class.get()>
                {children()}
            </menu>
        </div>
    }
}
