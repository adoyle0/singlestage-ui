use icondata::LuItalic;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleSmallExample() -> impl IntoView {
    view! {
        <Toggle size="small" attr:aria-label="Toggle italic">
            {icon!(LuItalic)}
        </Toggle>
    }
}
