use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaDisabledExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" disabled=true>
            <Label label_for="textarea-disabled">"Message"</Label>
            <Textarea id="textarea-disabled" placeholder="Type your message here." disabled=true />
        </Field>
    }
}
