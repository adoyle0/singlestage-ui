use icondata::LuItalic;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleWithTextExample() -> impl IntoView {
    view! {
        <Toggle aria_label="Toggle italic">
            {icon!(LuItalic)}
            "Italic"
        </Toggle>
    }
}
