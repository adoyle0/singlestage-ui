use crate::Button;
use leptos::prelude::*;

#[component]
pub fn SidebarMenuAction(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] show_on_hover: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <Button
            as_child=true
            class=format!(
                "singlestage-sidebar-menu-action{} {}",
                match show_on_hover.get_untracked().unwrap_or_default() {
                    false => "",
                    true => " singlestage-sidebar-menu-action-show-on-hover",
                },
                class.get().unwrap_or_default(),
            )
        >
            {children()}
        </Button>
    }
}
