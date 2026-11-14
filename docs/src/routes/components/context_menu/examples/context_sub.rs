use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextSubExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuItem>
                        "Copy" <ContextMenuShortcut>"⌘C"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Cut" <ContextMenuShortcut>"⌘X"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSub>
                    <ContextMenuSubTrigger>"More Tools"</ContextMenuSubTrigger>
                    <ContextMenuSubContent>
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
            </ContextMenuContent>
        </ContextMenu>
    }
}
