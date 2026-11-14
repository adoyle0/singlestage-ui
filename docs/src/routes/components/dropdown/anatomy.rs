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
                            <DropdownMenuCheckboxGroup>
                                <DropdownMenuCheckboxItem />
                            </DropdownMenuCheckboxGroup>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuRadioGroup>
                                <DropdownMenuRadioItem />
                            </DropdownMenuRadioGroup>
                        </DropdownMenuGroup>
                    </DropdownMenuSubContent>
                </DropdownMenuSub>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
