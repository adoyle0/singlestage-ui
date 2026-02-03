use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextCheckboxesExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <MenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <CheckboxItem checked=true>"Show Bookmarks Bar"</CheckboxItem>
                    <CheckboxItem>"Show Full URLs"</CheckboxItem>
                    <CheckboxItem checked=true>"Show Developer Tools"</CheckboxItem>
                </MenuGroup>
            </MenuContent>
        </ContextMenu>
    }
}
