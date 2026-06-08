use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-2 md:flex-row">
            <Button variant="outline">Button</Button>
            <Button variant="outline" size="icon" aria_label="Submit">
                {icon!(icondata::LuArrowUp)}
            </Button>
        </div>
    }
}
