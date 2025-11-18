use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugDropdown() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Dropdown Menu"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Console should log \"hey\" when clicked"</li>
            <li>"Console should log \"hey\" when tapped"</li>
            <li>
                "Console should log \"hey\" when selected with keyboard and "<Kbd>"Enter"</Kbd>
                " is pressed"
            </li>
        </ul>

        <DropdownMenu>
            <DropdownMenuTrigger>"Open"</DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem on:click=move |_| {
                    leptos::logging::log!("hey")
                }>"Click me"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
