use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownDestructiveExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <MenuTrigger>
                <Button variant="outline">"Actions"</Button>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>{icon!(icondata::LuPencil)} "Edit"</MenuItem>
                    <MenuItem>{icon!(icondata::LuShare)} "Share"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">{icon!(icondata::LuTrash)} "Delete"</MenuItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
