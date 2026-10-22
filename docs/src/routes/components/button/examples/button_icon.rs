use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonIconExample() -> impl IntoView {
    view! {
        <Button variant="outline" size="icon">
            {icon!(icondata::LuCircleFadingArrowUp)}
        </Button>
    }
}
