use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-6">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>{icon!(icondata::LuSearch)}</InputGroupAddon>
                <InputGroupAddon align="inline-end">"12 results"</InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="example.com" class="!pl-1" />
                <InputGroupAddon>
                    <InputGroupText>"https://"</InputGroupText>
                </InputGroupAddon>
                <InputGroupAddon align="inline-end">
                    <Tooltip value="This is content in a tooltip.">
                        <InputGroupButton class="rounded-full" size="icon-xs">
                            {icon!(icondata::LuInfo)}
                        </InputGroupButton>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupTextarea placeholder="Ask, Search or Chat..." />
                <InputGroupAddon align="block-end" class="max-h-[42px]">
                    <InputGroupButton variant="outline" class="rounded-full" size="icon-xs">
                        {icon!(icondata::LuPlus)}
                    </InputGroupButton>
                    <DropdownMenu>
                        <DropdownMenuTrigger variant="ghost">
                            "Auto"
                        // <InputGroupButton variant="ghost">"Auto"</InputGroupButton>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent class="[--radius:0.95rem]">
                            <DropdownMenuItem>"Auto"</DropdownMenuItem>
                            <DropdownMenuItem>"Agent"</DropdownMenuItem>
                            <DropdownMenuItem>"Manual"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                    <InputGroupText class="ml-auto">"52% used"</InputGroupText>
                    <Separator vertical=true class="!h-4" />
                    <InputGroupButton
                        variant="default"
                        class="rounded-full"
                        size="icon-xs"
                        disabled=true
                    >
                        {icon!(icondata::LuArrowUp)}
                        <span class="sr-only">"Send"</span>
                    </InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="@shadcn" />
                <InputGroupAddon align="inline-end">
                    <div class="bg-(--primary) text-(--primary-foreground) flex size-4 items-center justify-center rounded-full">
                        {icon!(icondata::LuCheck, class="size-3")}
                    </div>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
