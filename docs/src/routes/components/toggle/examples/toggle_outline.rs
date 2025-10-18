use icondata::LuItalic;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleOutlineExample() -> impl IntoView {
    view! {
        <Toggle variant="outline" aria_label="Toggle italic">
            {icon!(LuItalic)}
        </Toggle>
    }
}
