use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldTextareaExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldGroup>
                <Field>
                    <FieldLabel label_for="feedback">"Feedback"</FieldLabel>
                    <Textarea
                        id="feedback"
                        placeholder="Your feedback helps us improve..."
                        rows=4
                    />
                    <FieldDescription>"Share your thoughts about our service."</FieldDescription>
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
