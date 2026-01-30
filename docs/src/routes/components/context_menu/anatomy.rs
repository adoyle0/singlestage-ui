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
                            <CheckboxItem />
                            <CheckboxItem />
                        </ContextMenuGroup>
                        <ContextMenuSeparator />
                        <ContextMenuGroup>
                            <RadioGroup>
                                <RadioItem />
                                <RadioItem />
                                <RadioItem />
                            </RadioGroup>
                        </ContextMenuGroup>
                    </ContextMenuSubContent>
                </ContextMenuSub>
            </ContextMenuContent>
        </ContextMenu>
    }
}
