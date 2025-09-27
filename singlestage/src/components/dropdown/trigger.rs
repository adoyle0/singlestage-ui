use crate::{button::*, DropdownMenuContext};
use leptos::prelude::*;

/// The button that toggles the dropdown menu.
#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] variant: String,
) -> impl IntoView {
    let menu = expect_context::<DropdownMenuContext>();

    let uuid = uuid::Uuid::new_v4().to_string();
    menu.trigger_id.set(uuid.clone());

    view! {
        <Button
            attr:aria-controls=move || menu.menu_id.get()
            attr:aria-haspopup="menu"
            attr:popovertarget=move || menu.menu_id.get()
            attr:popovertargetaction="toggle"
            button_type="button"
            class=class
            id=uuid
            variant=variant
        >
            {children()}
        </Button>
    }
}
