use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextCheckboxesExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuCheckboxItem checked=true>
                        "Show Bookmarks Bar"
                    </ContextMenuCheckboxItem>
                    <ContextMenuCheckboxItem>"Show Full URLs"</ContextMenuCheckboxItem>
                    <ContextMenuCheckboxItem checked=true>
                        "Show Developer Tools"
                    </ContextMenuCheckboxItem>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
