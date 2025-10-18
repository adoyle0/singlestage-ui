use icondata::LuBold;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleExample() -> impl IntoView {
    view! {
        <Toggle aria_label="Toggle italic">
            {icon!(LuBold)}
        </Toggle>
    }
}
