use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupExample() -> impl IntoView {
    view! {
        <InputGroup class="max-w-xs">
            <Input placeholder="Search..." />
            <InputGroupAddon>{icon!(icondata::LuSearch)}</InputGroupAddon>
            <InputGroupAddon align="inline-end">"12 results"</InputGroupAddon>
        </InputGroup>
    }
}
