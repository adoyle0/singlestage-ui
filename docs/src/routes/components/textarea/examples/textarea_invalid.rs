use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaInvalidExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" invalid=true>
            <Label label_for="textarea-invalid">"Message"</Label>
            <Textarea id="textarea-invalid" placeholder="Type your message here." invalid=true />
            <FieldDescription>"Please enter a valid message."</FieldDescription>
        </Field>
    }
}
