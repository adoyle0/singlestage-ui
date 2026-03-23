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
            <MenuContent>
                <MenuGroup>
                    <MenuItem>"Back" <MenuShortcut>"⌘["</MenuShortcut></MenuItem>
                    <MenuItem disabled=true>"Forward" <MenuShortcut>"⌘]"</MenuShortcut></MenuItem>
                    <MenuItem>"Reload" <MenuShortcut>"⌘R"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>"Save" <MenuShortcut>"⌘S"</MenuShortcut></MenuItem>
                    <MenuItem>"Save As..." <MenuShortcut>"⇧⌘S"</MenuShortcut></MenuItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
