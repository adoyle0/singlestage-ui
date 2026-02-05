use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownIconsExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Open"</Button>
            </Trigger>
            <MenuContent>
                <MenuItem>{icon!(icondata::LuUser)} "Profile"</MenuItem>
                <MenuItem>{icon!(icondata::LuCreditCard)} "Billing"</MenuItem>
                <MenuItem>{icon!(icondata::LuSettings)} "Settings"</MenuItem>
                <Separator />
                <MenuItem variant="destructive">{icon!(icondata::LuLogOut)} "Log out"</MenuItem>
            </MenuContent>
        </DropdownMenu>
    }
}
