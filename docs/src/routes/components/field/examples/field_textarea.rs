use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldTextareaExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <Textarea placeholder="Your feedback helps us improve..." rows=4>
                            "Feedback"
                        </Textarea>
                        <FieldDescription>
                            "Share your thoughts about our service."
                        </FieldDescription>
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
