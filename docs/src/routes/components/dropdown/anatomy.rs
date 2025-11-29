use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownMenuAnatomy() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button />
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuLabel />
                    <DropdownMenuItem>
                        <DropdownMenuShortcut />
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
