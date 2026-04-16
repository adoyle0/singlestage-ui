use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipSideExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <For
                each=move || ["left", "top", "bottom", "right"]
                key=|side| side.to_string()
                let(side)
            >
                <Tooltip>
                    <TooltipTrigger>
                        <Button variant="outline" class="w-fit capitalize">
                            {side}
                        </Button>
                    </TooltipTrigger>
                    <TooltipContent side={side}>
                        <p>"Add to library"</p>
                    </TooltipContent>
                </Tooltip>
            </For>
        </div>
    }
}
