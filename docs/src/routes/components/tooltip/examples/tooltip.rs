use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipExample() -> impl IntoView {
    view! {
        <Tooltip value="Add to library">
            <Button variant="outline">"Hover"</Button>
        </Tooltip>
    }
}
