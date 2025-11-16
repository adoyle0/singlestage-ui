use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextMenuExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuGroup>
                    <ContextMenuLabel>"My Account"</ContextMenuLabel>
                    <ContextMenuItem>
                        "Profile" <ContextMenuShortcut>"⇧⌘P"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Billing" <ContextMenuShortcut>"⌘B"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Settings" <ContextMenuShortcut>"⌘S"</ContextMenuShortcut>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        "Keyboard shortcuts" <ContextMenuShortcut>"⌘K"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuItem>"GitHub"</ContextMenuItem>
                    <ContextMenuItem>"Support"</ContextMenuItem>
                    <ContextMenuItem disabled=true>"API"</ContextMenuItem>
                </ContextMenuGroup>
                <ContextMenuSeparator />
                <ContextMenuGroup>
                    <ContextMenuItem variant="destructive">
                        {icon!(icondata::LuTrash2)} "Delete Account"
                    </ContextMenuItem>
                    <ContextMenuItem>
                        {icon!(icondata::LuLogOut)} "Logout"
                        <ContextMenuShortcut>"⇧⌘Q"</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
