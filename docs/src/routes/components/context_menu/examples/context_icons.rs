use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextIconsExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>{icon!(icondata::LuCopy)} "Copy"</MenuItem>
                    <MenuItem>{icon!(icondata::LuScissors)} "Cut"</MenuItem>
                    <MenuItem>{icon!(icondata::LuClipboardPaste)} "Paste"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">{icon!(icondata::LuTrash)} "Delete"</MenuItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
