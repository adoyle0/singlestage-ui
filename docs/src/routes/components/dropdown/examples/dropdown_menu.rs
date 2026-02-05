use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownMenuExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Open"</Button>
            </Trigger>
            <MenuContent class="w-40" align="start">
                <MenuGroup>
                    <Label>"My Account"</Label>
                    <MenuItem>"Profile" <MenuShortcut>"⇧⌘P"</MenuShortcut></MenuItem>
                    <MenuItem>"Billing" <MenuShortcut>"⌘B"</MenuShortcut></MenuItem>
                    <MenuItem>"Settings" <MenuShortcut>"⌘S"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>"Team"</MenuItem>
                    <MenuSub>
                        <MenuSubTrigger>"Invite users"</MenuSubTrigger>
                        <MenuSubContent>
                            <MenuItem>"Email"</MenuItem>
                            <MenuItem>"Message"</MenuItem>
                            <Separator />
                            <MenuItem>"More..."</MenuItem>
                        </MenuSubContent>
                    </MenuSub>
                    <MenuItem>"New Team" <MenuShortcut>"⌘+T"</MenuShortcut></MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>"GitHub"</MenuItem>
                    <MenuItem>"Support"</MenuItem>
                    <MenuItem disabled=true>"API"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem>"Log out" <MenuShortcut>"⇧⌘Q"</MenuShortcut></MenuItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
