use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupAnatomy() -> impl IntoView {
    view! {
        <InputGroup>
            <Input />
            <InputGroupAddon>
                <InputGroupText />
            </InputGroupAddon>
        </InputGroup>
    }
}
