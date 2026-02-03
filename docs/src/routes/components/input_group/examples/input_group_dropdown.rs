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
                        <MenuTrigger>
                            <Button aria_label="More" title="More">
                                {icon!(icondata::FiMoreHorizontal)}
                            </Button>
                        </MenuTrigger>
                        <MenuContent align="end">
                            <MenuItem>"Settings"</MenuItem>
                            <MenuItem>"Copy path"</MenuItem>
                            <MenuItem>"Open location"</MenuItem>
                        </MenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup class="[--radius:1rem]">
                <Input placeholder="Enter search query" />
                <InputGroupAddon align="inline-end">
                    <DropdownMenu>
                        <MenuTrigger>
                            <Button class="!pr-1.5 text-xs">
                                "Search In..." {icon!(icondata::LuChevronDown, class="size-3")}
                            </Button>
                        </MenuTrigger>
                        <MenuContent align="end" class="[--radius:0.95rem]">
                            <MenuItem>"Documentation"</MenuItem>
                            <MenuItem>"Blog Posts"</MenuItem>
                            <MenuItem>"Changelog"</MenuItem>
                        </MenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
