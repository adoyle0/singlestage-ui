use leptos::prelude::*;
use singlestage::toggle::Toggle;

#[component]
pub fn ToggleAnatomy() -> impl IntoView {
    let pressed = create_rw_signal(false);
    view! { <Toggle pressed /> }
}
