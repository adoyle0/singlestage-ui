use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectExample() -> impl IntoView {
    view! {
        <Select>
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
