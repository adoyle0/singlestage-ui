use icondata::LuItalic;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleSmallExample() -> impl IntoView {
    view! {
        <Toggle size="sm" aria_label="Toggle italic">
            {icon!(LuItalic)}
        </Toggle>
    }
}
