use leptos::prelude::*;
use singlestage::select::*;

#[component]
pub fn SelectAnatomy() -> impl IntoView {
    view! {
        <Select class="w-[180px]">
            <optgroup label="Fruits">
                <option>"Apple"</option>
                <option>"Banana"</option>
                <option>"Blueberry"</option>
                <option>"Grapes"</option>
                <option>"Pineapple"</option>
            </optgroup>
        </Select>
    }
}
