use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TextareaFieldExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <FieldLabel>"Message"</FieldLabel>
            <FieldDescription>"Enter your message below."</FieldDescription>
            <Textarea placeholder="Type your message here." />
        </Field>
    }
}
