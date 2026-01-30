use leptos::prelude::*;
use singlestage::{Button, dropdown::*};

#[component]
pub fn DropdownSubExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuItem>"Team"</DropdownMenuItem>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Invite users"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuItem>"Email"</DropdownMenuItem>
                            <DropdownMenuItem>"Message"</DropdownMenuItem>
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger>"More options"</DropdownMenuSubTrigger>
                                <DropdownMenuSubContent>
                                    <DropdownMenuItem>"Calendly"</DropdownMenuItem>
                                    <DropdownMenuItem>"Slack"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem>"Webhook"</DropdownMenuItem>
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>"Advanced..."</DropdownMenuItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                    <DropdownMenuItem>
                        "New Team" <DropdownMenuShortcut>"⌘+T"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
