use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipExample() -> impl IntoView {
    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant="outline">"Hover"</Button>
            </TooltipTrigger>
            <TooltipContent>
                <p>"Add to library"</p>
            </TooltipContent>
        </Tooltip>
    }
}
