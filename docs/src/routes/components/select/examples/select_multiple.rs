use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectMultipleExample() -> impl IntoView {
    view! {
        <Select multiple=true class="h-32">
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
