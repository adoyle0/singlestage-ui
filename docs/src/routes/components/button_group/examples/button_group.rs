use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <ButtonGroup class="flex">
                <Button variant="outline" size="icon" attr:aria-label="Go Back">
                    {icon!(icondata::LuArrowLeft, width=24, height=24)}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline">"Archive"</Button>
                <Button variant="outline">"Report"</Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline">"Snooze"</Button>
                <DropdownMenu>
                    <DropdownMenuTrigger
                        variant="outline"
                        size="icon"
                        attr:aria-label="More Options"
                    >
                        {icon!(icondata::FiMoreHorizontal)}
                    </DropdownMenuTrigger>
                    <DropdownMenuContent class="w-52">
                        <DropdownMenuGroup>
                            <DropdownMenuItem>
                                {icon!(icondata::LuMailCheck)} "Mark as Read"
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                {icon!(icondata::LuArchive)} "Archive"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem>{icon!(icondata::LuClock)} "Snooze"</DropdownMenuItem>
                            <DropdownMenuItem>
                                {icon!(icondata::LuCalendarPlus)} "Add to Calendar"
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                {icon!(icondata::LuListPlus)} "Add to List"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem variant="destructive">
                                {icon!(icondata::LuTrash2)} "Trash"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                    </DropdownMenuContent>
                </DropdownMenu>
            </ButtonGroup>
        </ButtonGroup>
    }
}
