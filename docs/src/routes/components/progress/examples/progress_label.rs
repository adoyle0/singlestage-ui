use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ProgressLabelExample() -> impl IntoView {
    view! {
        <Field class="w-full max-w-sm">
            <FieldLabel>
                <span class="font-medium">"Upload progress"</span>
                <span class="ml-auto text-(--muted-foreground)">"66%"</span>
            </FieldLabel>
            <Progress value=66. />
        </Field>
    }
}
