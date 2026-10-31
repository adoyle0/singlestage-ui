use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectDisabledExample() -> impl IntoView {
    view! {
        <Select disabled=true>
            <SelectOption value="">"Disabled"</SelectOption>
            <SelectOption value="apple">"Apple"</SelectOption>
            <SelectOption value="banana">"Banana"</SelectOption>
            <SelectOption value="blueberry">"Blueberry"</SelectOption>
        </Select>
    }
}
