use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextDestructiveExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <Trigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </Trigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>{icon!(icondata::LuPencil)} "Edit"</MenuItem>
                    <MenuItem>{icon!(icondata::LuShare)} "Share"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">{icon!(icondata::LuTrash)} "Delete"</MenuItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
