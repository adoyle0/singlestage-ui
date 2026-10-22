use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputBasicExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Input placeholder="Enter text" />
        </Field>
    }
}
