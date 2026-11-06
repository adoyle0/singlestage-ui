use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputInlineExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" orientation="horizontal">
            <Input placeholder="Search..." />
            <Button>"Search"</Button>
        </Field>
    }
}
