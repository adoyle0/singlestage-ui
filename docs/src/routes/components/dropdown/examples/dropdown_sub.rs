use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownSubExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <MenuTrigger>
                <Button variant="outline">"Open"</Button>
            </MenuTrigger>
            <MenuContent>
                <MenuGroup>
                    <MenuItem>"Team"</MenuItem>
                    <MenuSub>
                        <MenuSubTrigger>"Invite users"</MenuSubTrigger>
                        <MenuSubContent>
                            <MenuItem>"Email"</MenuItem>
                            <MenuItem>"Message"</MenuItem>
                            <MenuSub>
                                <MenuSubTrigger>"More options"</MenuSubTrigger>
                                <MenuSubContent>
                                    <MenuItem>"Calendly"</MenuItem>
                                    <MenuItem>"Slack"</MenuItem>
                                    <Separator />
                                    <MenuItem>"Webhook"</MenuItem>
                                </MenuSubContent>
                            </MenuSub>
                            <Separator />
                            <MenuItem>"Advanced..."</MenuItem>
                        </MenuSubContent>
                    </MenuSub>
                    <MenuItem>"New Team" <MenuShortcut>"⌘+T"</MenuShortcut></MenuItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
