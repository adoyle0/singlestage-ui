use leptos::prelude::*;
use singlestage::dropdown::*;

#[component]
pub fn DropdownMenuExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger variant="outline">"Open"</DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                    <DropdownMenuItem>
                        "Profile" <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Billing" <DropdownMenuShortcut>"⌘B"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Settings" <DropdownMenuShortcut>"⌘S"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Keyboard shortcuts" <DropdownMenuShortcut>"⌘K"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"GitHub"</DropdownMenuItem>
                <DropdownMenuItem>"Support"</DropdownMenuItem>
                <DropdownMenuItem disabled=true>"API"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>
                    "Logout" <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
