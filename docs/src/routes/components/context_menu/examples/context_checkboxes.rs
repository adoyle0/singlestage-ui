use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextCheckboxesExample() -> impl IntoView {
    let value = RwSignal::new(vec!["bookmarks".to_string(), "devtools".to_string()]);

    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-xl border border-dashed text-sm">
                <span class="pointer-fine:hidden">"Long press here"</span>
                <span class="pointer-coarse:hidden">"Right click here"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuCheckboxGroup value>
                        <ContextMenuCheckboxItem value="bookmarks">
                            "Show Bookmarks Bar"
                        </ContextMenuCheckboxItem>
                        <ContextMenuCheckboxItem value="urls">
                            "Show Full URLs"
                        </ContextMenuCheckboxItem>
                        <ContextMenuCheckboxItem value="devtools">
                            "Show Developer Tools"
                        </ContextMenuCheckboxItem>
                    </ContextMenuCheckboxGroup>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
