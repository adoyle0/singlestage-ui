use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchDisabledExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" disabled=true class="w-fit">
            <Switch>"Disabled"</Switch>
        </Field>
    }
}
