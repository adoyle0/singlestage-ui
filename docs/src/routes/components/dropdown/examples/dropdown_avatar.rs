use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownAvatarExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="ghost" size="icon" class="rounded-full">
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" alt="shadcn" />
                        <AvatarFallback>"LR"</AvatarFallback>
                    </Avatar>
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
                <DropdownMenuGroup>
                    <DropdownMenuItem>{icon!(icondata::LuBadgeCheck)} "Account"</DropdownMenuItem>
                    <DropdownMenuItem>{icon!(icondata::LuCreditCard)} "Billing"</DropdownMenuItem>
                    <DropdownMenuItem>{icon!(icondata::LuBell)} "Notifications"</DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuItem>{icon!(icondata::LuLogOut)} "Sign Out"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
