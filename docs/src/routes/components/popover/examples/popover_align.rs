use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverAlignExample() -> impl IntoView {
    view! {
        <div class="flex gap-6">
            <Popover>
                <Trigger>
                    <Button variant="outline" size="sm">
                        "Start"
                    </Button>
                </Trigger>
                <PopoverContent align="start" class="w-40">
                    "Aligned to start"
                </PopoverContent>
            </Popover>
            <Popover>
                <Trigger>
                    <Button variant="outline" size="sm">
                        "Center"
                    </Button>
                </Trigger>
                <PopoverContent align="center" class="w-40">
                    "Aligned to center"
                </PopoverContent>
            </Popover>
            <Popover>
                <Trigger>
                    <Button variant="outline" size="sm">
                        "End"
                    </Button>
                </Trigger>
                <PopoverContent align="end" class="w-40">
                    "Aligned to end"
                </PopoverContent>
            </Popover>
        </div>
    }
}
