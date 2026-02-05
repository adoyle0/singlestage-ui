use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownBasicExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Open"</Button>
            </Trigger>
            <MenuContent>
                <MenuGroup>
                    <Label>"My Account"</Label>
                    <MenuItem>"Profile"</MenuItem>
                    <MenuItem>"Billing"</MenuItem>
                    <MenuItem>"Settings"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuItem>"GitHub"</MenuItem>
                <MenuItem>"Support"</MenuItem>
                <MenuItem disabled=true>"API"</MenuItem>
            </MenuContent>
        </DropdownMenu>
    }
}
