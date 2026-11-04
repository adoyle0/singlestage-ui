use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SwitchDescriptionExample() -> impl IntoView {
    view! {
        <Field orientation="horizontal" class="max-w-sm">
            <FieldContent>
                <FieldLabel>"Share across devices"</FieldLabel>
                <FieldDescription>
                    "Focus is shared across devices, and turns off when you leave the app."
                </FieldDescription>
            </FieldContent>
            <Switch />
        </Field>
    }
}
