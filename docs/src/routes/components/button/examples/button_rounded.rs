use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonRoundedExample() -> impl IntoView {
    view! {
        <Button variant="outline" size="icon" class="rounded-full">
            {icon!(icondata::LuArrowUp)}
        </Button>
    }
}
