use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupInputExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Input placeholder="Search..." />
            <Button variant="outline" aria_label="Search">
                {icon!(icondata::LuSearch)}
            </Button>
        </ButtonGroup>
    }
}
