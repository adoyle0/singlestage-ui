use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaDisabledExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" disabled=true>
            <Textarea placeholder="Type your message here.">"Message"</Textarea>
        </Field>
    }
}
