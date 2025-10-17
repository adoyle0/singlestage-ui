use icondata::LuItalic;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleLargeExample() -> impl IntoView {
    view! {
        <Toggle size="large" attr:aria-label="Toggle italic">
            {icon!(LuItalic)}
        </Toggle>
    }
}
