use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupExample() -> impl IntoView {
    let label = RwSignal::new("personal".to_string());

    view! {
        <ButtonGroup>
            <ButtonGroup class="flex">
                <Button variant="outline" size="icon" aria_label="Go Back">
                    {icon!(icondata::LuArrowLeft)}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline">"Archive"</Button>
                <Button variant="outline">"Report"</Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline">"Snooze"</Button>
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <Button variant="outline" size="icon" aria_label="More Options">
                            {icon!(icondata::FiMoreHorizontal)}
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
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
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger>
                                    {icon!(icondata::LuTag)} "Label As..."
                                </DropdownMenuSubTrigger>
                                <DropdownMenuSubContent>
                                    <RadioGroup value=label>
                                        <RadioItem value="personal">"Personal"</RadioItem>
                                        <RadioItem value="work">"Work"</RadioItem>
                                        <RadioItem value="other">"Other"</RadioItem>
                                    </RadioGroup>
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
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
