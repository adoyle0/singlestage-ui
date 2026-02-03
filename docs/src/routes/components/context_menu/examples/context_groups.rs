use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextGroupsExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <Label>"File"</Label>
                    <MenuItem>"New File" <MenuShortcut>"⌘N"</MenuShortcut></MenuItem>
                    <MenuItem>"Open File" <MenuShortcut>"⌘O"</MenuShortcut></MenuItem>
                    <MenuItem>"Save" <MenuShortcut>"⌘S"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <Label>"Edit"</Label>
                    <MenuItem>"Undo" <MenuShortcut>"⌘Z"</MenuShortcut></MenuItem>
                    <MenuItem>"Redo" <MenuShortcut>"⇧⌘Z"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>"Cut" <MenuShortcut>"⌘X"</MenuShortcut></MenuItem>
                    <MenuItem>"Copy" <MenuShortcut>"⌘C"</MenuShortcut></MenuItem>
                    <MenuItem>"Paste" <MenuShortcut>"⌘V"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">
                        "Delete" <MenuShortcut>"⌫"</MenuShortcut>
                    </MenuItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
