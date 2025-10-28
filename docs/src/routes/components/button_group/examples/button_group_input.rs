use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupInputExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Input placeholder="Search..." />
            <Button variant="outline" attr:aria-label="Search">
                {icon!(icondata::LuSearch)}
            </Button>
        </ButtonGroup>
    }
}
