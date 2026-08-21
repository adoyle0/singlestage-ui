use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldTextareaExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Textarea placeholder="Your feedback helps us improve..." rows=4>
                "Feedback"
            </Textarea>
            <FieldDescription>"Share your thoughts about our service."</FieldDescription>
        </Field>
    }
}
