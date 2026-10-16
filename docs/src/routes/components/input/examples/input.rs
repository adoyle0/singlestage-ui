use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Input input_type="password" placeholder="sk-..." />
            <FieldDescription>"Your API key is encrypted and stored securely."</FieldDescription>
        </Field>
    }
}
