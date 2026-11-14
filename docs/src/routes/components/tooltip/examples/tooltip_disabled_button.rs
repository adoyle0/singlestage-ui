use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipDisabledButtonExample() -> impl IntoView {
    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant="outline" disabled=true>
                    "Disabled"
                </Button>
            </TooltipTrigger>
            <TooltipContent>
                <p>"This feature is currently unavailable"</p>
            </TooltipContent>
        </Tooltip>
    }
}
