use icondata::LuBold;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleExample() -> impl IntoView {
    view! { <Toggle attr:aria-label="Toggle italic">{icon!(LuBold)}</Toggle> }
}
