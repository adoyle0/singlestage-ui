use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextGroupsExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuLabel>"File"</ContextMenuLabel>
                    <ContextMenuItem>
                        "New File" <ContextMenuShortcut>"⌘N"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Open File" <ContextMenuShortcut>"⌘O"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Save" <ContextMenuShortcut>"⌘S"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuLabel>"Edit"</ContextMenuLabel>
                    <ContextMenuItem>
                        "Undo" <ContextMenuShortcut>"⌘Z"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Redo" <ContextMenuShortcut>"⇧⌘Z"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuItem>
                        "Cut" <ContextMenuShortcut>"⌘X"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Copy" <ContextMenuShortcut>"⌘C"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Paste" <ContextMenuShortcut>"⌘V"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuItem variant="destructive">
                        "Delete" <ContextMenuShortcut>"⌫"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
