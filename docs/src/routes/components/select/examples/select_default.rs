use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectDefaultExample() -> impl IntoView {
    view! {
        <Select default="pineapple">
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
