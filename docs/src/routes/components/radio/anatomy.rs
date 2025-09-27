use leptos::prelude::*;
use singlestage::radio::*;

#[component]
pub fn RadioAnatomy() -> impl IntoView {
    view! {
        <RadioGroup>
            <Radio />
        </RadioGroup>
    }
}
