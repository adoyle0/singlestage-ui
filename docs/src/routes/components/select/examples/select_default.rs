use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectDefaultExample() -> impl IntoView {
    view! {
        <Select default="pineapple">
            <SelectOptGroup label="Fruits">
                <SelectOption value="apple">"Apple"</SelectOption>
                <SelectOption value="banana">"Banana"</SelectOption>
                <SelectOption value="blueberry">"Blueberry"</SelectOption>
                <SelectOption value="grapes">"Grapes"</SelectOption>
                <SelectOption value="pineapple">"Pineapple"</SelectOption>
            </SelectOptGroup>
        </Select>
    }
}
