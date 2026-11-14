use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextShortcutsExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
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
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuItem>
                        "Save" <ContextMenuShortcut>"⌘S"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Save As..." <ContextMenuShortcut>"⇧⌘S"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
