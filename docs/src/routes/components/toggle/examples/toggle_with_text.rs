use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleWithTextExample() -> impl IntoView {
    view! {
        <Toggle aria_label="Toggle italic">
            {icon!(icondata::LuItalic)} <span>"Italic"</span>
        </Toggle>
    }
}
