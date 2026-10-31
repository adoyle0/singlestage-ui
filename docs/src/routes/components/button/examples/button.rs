use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Button variant="outline">Button</Button>
            <Button variant="outline" size="icon" aria_label="Submit">
                {icon!(icondata::LuArrowUp)}
            </Button>
        </div>
    }
}
