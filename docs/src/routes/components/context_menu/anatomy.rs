use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ContextMenuAnatomy() -> impl IntoView {
    view! {
        <ContextMenu>
            <Trigger />
            <MenuContent>
                <MenuGroup>
                    <Label />
                    <MenuItem>
                        <MenuShortcut />
                    </MenuItem>
                </MenuGroup>
                <Separator />
                <MenuSub>
                    <MenuSubTrigger />
                    <MenuSubContent>
                        <MenuGroup>
                            <CheckboxItem />
                            <CheckboxItem />
                        </MenuGroup>
                        <Separator />
                        <MenuGroup>
                            <RadioGroup>
                                <RadioItem />
                                <RadioItem />
                                <RadioItem />
                            </RadioGroup>
                        </MenuGroup>
                    </MenuSubContent>
                </MenuSub>
            </MenuContent>
        </ContextMenu>
    }
}
