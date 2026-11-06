use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectExample() -> impl IntoView {
    view! {
        <Select>
            <SelectOption value="">"Select status"</SelectOption>
            <SelectOption value="todo">"Todo"</SelectOption>
            <SelectOption value="in-progress">"In Progress"</SelectOption>
            <SelectOption value="done">"Done"</SelectOption>
            <SelectOption value="cancelled">"Cancelled"</SelectOption>
        </Select>
    }
}
