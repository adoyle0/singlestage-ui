use leptos::prelude::*;
use singlestage::context_menu::*;

#[component]
pub fn ContextMenuAnatomy() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger />
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuLabel />
                    <ContextMenuItem>
                        <ContextMenuShortcut />
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
            </ContextMenuContent>
        </ContextMenu>
    }
}
