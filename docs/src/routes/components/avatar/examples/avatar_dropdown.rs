use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarDropdownExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="ghost" size="icon" class="rounded-full">
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" alt="shadcn" />
                        <AvatarFallback>"CN"</AvatarFallback>
                    </Avatar>
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-32">
                <DropdownMenuGroup>
                    <DropdownMenuItem>"Profile"</DropdownMenuItem>
                    <DropdownMenuItem>"Billing"</DropdownMenuItem>
                    <DropdownMenuItem>"Settings"</DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem variant="destructive">"Log out"</DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
