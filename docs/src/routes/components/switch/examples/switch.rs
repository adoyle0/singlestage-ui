use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchExample() -> impl IntoView {
    view! {
        <Label>
            <Switch name="switch" />
            "Airplane Mode"
        </Label>
    }
}
