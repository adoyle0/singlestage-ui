use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupDropdownExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <Input placeholder="Enter file name" />
                <InputGroupAddon align="inline-end">
                    <DropdownMenu>
                        <DropdownMenuTrigger aria_label="More" title="More" variant="ghost">
                            {icon!(icondata::FiMoreHorizontal)}
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuItem>"Settings"</DropdownMenuItem>
                            <DropdownMenuItem>"Copy path"</DropdownMenuItem>
                            <DropdownMenuItem>"Open location"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup class="[--radius:1rem]">
                <Input placeholder="Enter search query" />
                <InputGroupAddon align="inline-end">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant="ghost" class="!pr-1.5 text-xs">
                            "Search In..."
                            {icon!(icondata::LuChevronDown, class="size-3")}
                        </DropdownMenuTrigger>
                        <DropdownMenuContent class="[--radius:0.95rem]">
                            <DropdownMenuItem>"Documentation"</DropdownMenuItem>
                            <DropdownMenuItem>"Blog Posts"</DropdownMenuItem>
                            <DropdownMenuItem>"Changelog"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
