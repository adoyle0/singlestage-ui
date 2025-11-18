use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectPlaceholderExample() -> impl IntoView {
    view! {
        <Select placeholder="Select your favorite fruit">
            <SelectContent label="Fruits">
                <SelectItem value="apple">"Apple"</SelectItem>
                <SelectItem value="banana">"Banana"</SelectItem>
                <SelectItem value="blueberry">"Blueberry"</SelectItem>
                <SelectItem value="grapes">"Grapes"</SelectItem>
                <SelectItem value="pineapple">"Pineapple"</SelectItem>
            </SelectContent>
        </Select>
    }
}
