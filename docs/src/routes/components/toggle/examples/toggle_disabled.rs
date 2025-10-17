use icondata::LuUnderline;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleDisabledExample() -> impl IntoView {
    view! {
        <Toggle attr:disabled attr:aria-label="Toggle italic">
            {icon!(LuUnderline)}
        </Toggle>
    }
}

