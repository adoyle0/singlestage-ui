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
                    <MenuTrigger>
                        <Button variant="outline" size="icon" aria_label="More Options">
                            {icon!(icondata::FiMoreHorizontal)}
                        </Button>
                    </MenuTrigger>
                    <MenuContent align="end">
                        <MenuGroup>
                            <MenuItem>{icon!(icondata::LuMailCheck)} "Mark as Read"</MenuItem>
                            <MenuItem>{icon!(icondata::LuArchive)} "Archive"</MenuItem>
                        </MenuGroup>
                        <Separator />
                        <MenuGroup>
                            <MenuItem>{icon!(icondata::LuClock)} "Snooze"</MenuItem>
                            <MenuItem>{icon!(icondata::LuCalendarPlus)} "Add to Calendar"</MenuItem>
                            <MenuItem>{icon!(icondata::LuListPlus)} "Add to List"</MenuItem>
                            <MenuSub>
                                <MenuSubTrigger>
                                    {icon!(icondata::LuTag)} "Label As..."
                                </MenuSubTrigger>
                                <MenuSubContent>
                                    <RadioGroup value=label>
                                        <RadioItem value="personal">"Personal"</RadioItem>
                                        <RadioItem value="work">"Work"</RadioItem>
                                        <RadioItem value="other">"Other"</RadioItem>
                                    </RadioGroup>
                                </MenuSubContent>
                            </MenuSub>
                        </MenuGroup>
                        <Separator />
                        <MenuGroup>
                            <MenuItem variant="destructive">
                                {icon!(icondata::LuTrash2)} "Trash"
                            </MenuItem>
                        </MenuGroup>
                    </MenuContent>
                </DropdownMenu>
            </ButtonGroup>
        </ButtonGroup>
    }
}
