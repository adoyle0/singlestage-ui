use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarDropdownExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="ghost" size="icon" class="rounded-full">
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" alt="shadcn" />
                        <AvatarFallback>"CN"</AvatarFallback>
                    </Avatar>
                </Button>
            </Trigger>
            <MenuContent class="w-32">
                <MenuGroup>
                    <MenuItem>"Profile"</MenuItem>
                    <MenuItem>"Billing"</MenuItem>
                    <MenuItem>"Settings"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuGroup>
                    <MenuItem variant="destructive">"Log out"</MenuItem>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
