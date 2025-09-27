use crate::DropdownMenuGroupContext;
use leptos::prelude::*;

/// Labels groups.
#[component]
pub fn DropdownMenuLabel(children: Children) -> impl IntoView {
    let group = expect_context::<DropdownMenuGroupContext>();

    let uuid = uuid::Uuid::new_v4().to_string();
    group.heading_id.set(uuid.clone());

    view! {
        <h6 class="singlestage-dropdown-menu-label" id=uuid>
            {children()}
        </h6>
    }
}
