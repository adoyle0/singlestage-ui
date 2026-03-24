use leptos::prelude::*;
use singlestage::*;

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
                <ContextMenuSub>
                    <ContextMenuSubTrigger />
                    <ContextMenuSubContent>
                        <ContextMenuGroup>
                            <ContextMenuCheckboxItem />
                            <ContextMenuCheckboxItem />
                        </ContextMenuGroup>
                        <ContextMenuSeparator />
                        <ContextMenuGroup>
                            <ContextMenuRadioGroup>
                                <ContextMenuRadioItem />
                                <ContextMenuRadioItem />
                                <ContextMenuRadioItem />
                            </ContextMenuRadioGroup>
                        </ContextMenuGroup>
                    </ContextMenuSubContent>
                </ContextMenuSub>
            </ContextMenuContent>
        </ContextMenu>
    }
}
