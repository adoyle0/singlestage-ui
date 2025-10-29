use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupPopoverExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="outline">{icon!(icondata::LuBot)} "Copilot"</Button>
            <Popover>
                <PopoverTrigger variant="outline" size="icon" attr:aria-label="Open Popover">
                    {icon!(icondata::LuChevronDown)}
                </PopoverTrigger>
                <PopoverContent class="rounded-xl p-0 text-sm max-w-72">
                    <p class="px-4 pt-3 text-sm font-medium">"Agent Tasks"</p>
                    <Separator />
                    <div class="p-4 pt-0 text-sm *:[p:not(:last-child)]:mb-2">
                        <Textarea
                            placeholder="Describe your task in natural language."
                            class="mb-4 resize-none"
                        />
                        <p class="font-medium">"Start a new task with Copilot"</p>
                        <p class="text-(--muted-foreground)">
                            "Describe your task in natural language. Copilot will work in the
                            background and open a pull request for your review."
                        </p>
                    </div>
                </PopoverContent>
            </Popover>
        </ButtonGroup>
    }
}
