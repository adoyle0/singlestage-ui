use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownMenuAnatomy() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button />
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuLabel />
                    <DropdownMenuItem>
                        <DropdownMenuShortcut />
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuSub>
                    <DropdownMenuSubTrigger />
                    <DropdownMenuSubContent>
                        <DropdownMenuGroup>
                            <CheckboxItem />
                            <CheckboxItem />
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <RadioGroup>
                                <RadioItem />
                                <RadioItem />
                                <RadioItem />
                            </RadioGroup>
                        </DropdownMenuGroup>
                    </DropdownMenuSubContent>
                </DropdownMenuSub>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
