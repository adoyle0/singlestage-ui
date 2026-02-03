use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownShortcutsExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <MenuTrigger>
                <Button variant="outline">"Open"</Button>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <Label>"My Account"</Label>
                    <MenuItem>"Profile" <MenuShortcut>"⇧⌘P"</MenuShortcut></MenuItem>
                    <MenuItem>"Billing" <MenuShortcut>"⌘B"</MenuShortcut></MenuItem>
                    <MenuItem>"Settings" <MenuShortcut>"⌘S"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuItem>"Log out" <MenuShortcut>"⇧⌘Q"</MenuShortcut></MenuItem>
            </MenuContent>
        </DropdownMenu>
    }
}
