use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaFieldExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Label label_for="textarea-message">"Message"</Label>
            <FieldDescription>"Enter your message below."</FieldDescription>
            <Textarea id="textarea-message" placeholder="Type your message here." />
        </Field>
    }
}
