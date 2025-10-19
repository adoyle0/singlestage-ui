use icondata::LuUnderline;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleDisabledExample() -> impl IntoView {
    view! {
        <Toggle disabled=true aria_label="Toggle italic">
            {icon!(LuUnderline)}
        </Toggle>
    }
}
