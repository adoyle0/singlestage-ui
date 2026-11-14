use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugContextMenu() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Context Menu"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Console should log \"hey\" when clicked"</li>
            <li>"Console should log \"hey\" when tapped"</li>
            <li>
                "Console should log \"hey\" when selected with keyboard and "<Kbd>"Enter"</Kbd>
                " is pressed"
            </li>
        </ul>

        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem on:click=move |_| {
                    leptos::logging::log!("hey")
                }>"Click me"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
}
