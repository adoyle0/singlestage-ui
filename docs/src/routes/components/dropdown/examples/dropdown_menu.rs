use leptos::prelude::*;
use singlestage::{Button, dropdown::*, icon};

#[component]
pub fn DropdownMenuExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
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
                <DropdownMenuGroup>
                    <DropdownMenuItem>"GitHub"</DropdownMenuItem>
                    <DropdownMenuItem>"Support"</DropdownMenuItem>
                    <DropdownMenuItem disabled=true>"API"</DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem variant="destructive">
                        {icon!(icondata::LuTrash2)} "Delete Account"
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        {icon!(icondata::LuLogOut)} "Logout"
                        <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
