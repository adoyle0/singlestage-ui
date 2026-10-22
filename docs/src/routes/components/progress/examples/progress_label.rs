use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ProgressLabelExample() -> impl IntoView {
    view! {
        <Field class="w-full max-w-sm">
            <Label label_for="progress-upload">
                <span>"Upload progress"</span>
                <span class="ml-auto">"66%"</span>
            </Label>
            <Progress value=66. id="progress-upload" />
        </Field>
    }
}
