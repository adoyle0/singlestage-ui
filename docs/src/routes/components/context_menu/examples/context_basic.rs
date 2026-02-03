use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextBasicExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>"Back"</MenuItem>
                    <MenuItem disabled=true>"Forward"</MenuItem>
                    <MenuItem>"Reload"</MenuItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
