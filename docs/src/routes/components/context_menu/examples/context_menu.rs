use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextMenuExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent class="w-48">
                <ContextMenuGroup>
                    <ContextMenuItem>
                        "Back" <ContextMenuShortcut>"⌘["</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem disabled=true>
                        "Forward" <ContextMenuShortcut>"⌘]"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Reload" <ContextMenuShortcut>"⌘R"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"More Tools"</ContextMenuSubTrigger>
                        <ContextMenuSubContent class="w-44">
                            <ContextMenuGroup>
                                <ContextMenuItem>"Save Page..."</ContextMenuItem>
                                <ContextMenuItem>"Create Shortcut..."</ContextMenuItem>
                                <ContextMenuItem>"Name Window..."</ContextMenuItem>
                            </ContextMenuGroup>
                            <ContextMenuSeparator />
                            <ContextMenuGroup>
                                <ContextMenuItem>"Developer Tools"</ContextMenuItem>
                            </ContextMenuGroup>
                            <ContextMenuSeparator />
                            <ContextMenuGroup>
                                <ContextMenuItem variant="destructive">"Delete"</ContextMenuItem>
                            </ContextMenuGroup>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuCheckboxItem checked=true>"Show Bookmarks"</ContextMenuCheckboxItem>
                    <ContextMenuCheckboxItem>"Show Full URLs"</ContextMenuCheckboxItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuRadioGroup value="pedro">
                        <ContextMenuLabel>"People"</ContextMenuLabel>
                        <ContextMenuRadioItem value="pedro">"Pedro Duarte"</ContextMenuRadioItem>
                        <ContextMenuRadioItem value="colm">"Colm Tuite"</ContextMenuRadioItem>
                    </ContextMenuRadioGroup>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
