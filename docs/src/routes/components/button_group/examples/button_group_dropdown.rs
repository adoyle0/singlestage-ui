use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupDropdownExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="outline">"Follow"</Button>
            <DropdownMenu>
                <MenuTrigger>
                    <Button variant="outline" class="!px-2">
                        {icon!(icondata::LuChevronDown)}
                    </Button>
                </MenuTrigger>
                <MenuContent align="end">
                    <MenuGroup>
                        <MenuItem>
                            {icon!(icondata::BiVolumeMuteRegular)} "Mute Conversation"
                        </MenuItem>
                        <MenuItem>{icon!(icondata::LuCheck)} "Mark as Read"</MenuItem>
                        <MenuItem>
                            {icon!(icondata::FiAlertTriangle)} "Report Conversation"
                        </MenuItem>
                        <MenuItem>{icon!(icondata::LuUserX)} "Block User"</MenuItem>
                        <MenuItem>{icon!(icondata::LuShare)} "Share Conversation"</MenuItem>
                        <MenuItem>{icon!(icondata::LuCopy)} "Copy Conversation"</MenuItem>
                    </MenuGroup>
                    <Separator />
                    <MenuGroup>
                        <MenuItem variant="destructive">
                            {icon!(icondata::LuTrash)}"Delete Conversation"
                        </MenuItem>
                    </MenuGroup>
                </MenuContent>
            </DropdownMenu>
        </ButtonGroup>
    }
}
