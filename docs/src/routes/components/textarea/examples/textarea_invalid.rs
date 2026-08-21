use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaInvalidExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" invalid=true>
            <Textarea placeholder="Type your message here.">"Message"</Textarea>
            <FieldDescription>"Please enter a valid message."</FieldDescription>
        </Field>
    }
}
