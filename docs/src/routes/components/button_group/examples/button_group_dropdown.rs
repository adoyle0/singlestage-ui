use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupDropdownExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="outline">"Follow"</Button>
            <DropdownMenu>
                <DropdownMenuTrigger variant="outline" class="!pl-2">
                    {icon!(icondata::LuChevronDown)}
                </DropdownMenuTrigger>
                <DropdownMenuContent class="[--radius:1rem]">
                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            {icon!(icondata::BiVolumeMuteRegular)} "Mute Conversation"
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            {icon!(icondata::LuCheck)} "Mark as Read"
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            {icon!(icondata::FiAlertTriangle)} "Report Conversation"
                        </DropdownMenuItem>
                        <DropdownMenuItem>{icon!(icondata::LuUserX)} "Block User"</DropdownMenuItem>
                        <DropdownMenuItem>
                            {icon!(icondata::LuShare)} "Share Conversation"
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            {icon!(icondata::LuCopy)} "Copy Conversation"
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                    <DropdownMenuSeparator />
                    <DropdownMenuGroup>
                        <DropdownMenuItem variant="destructive">
                            {icon!(icondata::LuTrash)}"Delete Conversation"
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </ButtonGroup>
    }
}
