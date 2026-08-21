use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownBasicExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                    <DropdownMenuItem>"Profile"</DropdownMenuItem>
                    <DropdownMenuItem>"Billing"</DropdownMenuItem>
                    <DropdownMenuItem>"Settings"</DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"GitHub"</DropdownMenuItem>
                <DropdownMenuItem>"Support"</DropdownMenuItem>
                <DropdownMenuItem disabled=true>"API"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
