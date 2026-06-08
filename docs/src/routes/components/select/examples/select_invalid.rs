use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectInvalidExample() -> impl IntoView {
    view! {
        <Select invalid=true>
            <SelectOption value="">"Error state"</SelectOption>
            <SelectOption value="apple">"Apple"</SelectOption>
            <SelectOption value="banana">"Banana"</SelectOption>
            <SelectOption value="blueberry">"Blueberry"</SelectOption>
        </Select>
    }
}
