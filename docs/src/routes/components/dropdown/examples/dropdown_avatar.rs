use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownAvatarExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="ghost" size="icon" class="rounded-full">
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" alt="shadcn" />
                        <AvatarFallback>"LR"</AvatarFallback>
                    </Avatar>
                </Button>
            </Trigger>
            <MenuContent align="end">
                <MenuGroup>
                    <MenuItem>{icon!(icondata::LuBadgeCheck)} "Account"</MenuItem>
                    <MenuItem>{icon!(icondata::LuCreditCard)} "Billing"</MenuItem>
                    <MenuItem>{icon!(icondata::LuBell)} "Notifications"</MenuItem>
                </MenuGroup>
                <Separator />
                <MenuItem>{icon!(icondata::LuLogOut)} "Sign Out"</MenuItem>
            </MenuContent>
        </DropdownMenu>
    }
}
