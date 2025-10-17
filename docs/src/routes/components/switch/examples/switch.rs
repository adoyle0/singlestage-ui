use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchExample() -> impl IntoView {
    view! { <Switch checked=true>"Airplane Mode"</Switch> }
}
