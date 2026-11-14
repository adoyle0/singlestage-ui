use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownDestructiveExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Actions"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuItem>{icon!(icondata::LuPencil)} "Edit"</DropdownMenuItem>
                    <DropdownMenuItem>{icon!(icondata::LuShare)} "Share"</DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem variant="destructive">
                        {icon!(icondata::LuTrash)} "Delete"
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
