use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownMenuExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40" align="start">
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
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem>"Team"</DropdownMenuItem>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Invite users"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuItem>"Email"</DropdownMenuItem>
                            <DropdownMenuItem>"Message"</DropdownMenuItem>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>"More..."</DropdownMenuItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                    <DropdownMenuItem>
                        "New Team" <DropdownMenuShortcut>"⌘+T"</DropdownMenuShortcut>
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
                    <DropdownMenuItem>
                        "Log out" <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
