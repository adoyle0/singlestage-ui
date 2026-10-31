use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownIconsExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>{icon!(icondata::LuUser)} "Profile"</DropdownMenuItem>
                <DropdownMenuItem>{icon!(icondata::LuCreditCard)} "Billing"</DropdownMenuItem>
                <DropdownMenuItem>{icon!(icondata::LuSettings)} "Settings"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem variant="destructive">
                    {icon!(icondata::LuLogOut)} "Log out"
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
