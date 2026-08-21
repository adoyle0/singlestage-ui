use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextMenuAnatomy() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger>
                <Button />
            </ContextMenuTrigger>
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
                            <ContextMenuCheckboxGroup>
                                <ContextMenuCheckboxItem />
                            </ContextMenuCheckboxGroup>
                        </ContextMenuGroup>
                        <ContextMenuSeparator />
                        <ContextMenuGroup>
                            <ContextMenuRadioGroup>
                                <ContextMenuRadioItem />
                            </ContextMenuRadioGroup>
                        </ContextMenuGroup>
                    </ContextMenuSubContent>
                </ContextMenuSub>
            </ContextMenuContent>
        </ContextMenu>
    }
}
