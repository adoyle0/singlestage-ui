use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioDescriptionExample() -> impl IntoView {
    view! {
        <RadioGroup default="comfortable" class="w-fit">
            <Field orientation="horizontal">
                <Radio value="default" />
                <FieldContent>
                    <FieldLabel>"Default"</FieldLabel>
                    <FieldDescription>"Standard spacing for most use cases."</FieldDescription>
                </FieldContent>
            </Field>
            <Field orientation="horizontal">
                <Radio value="comfortable" />
                <FieldContent>
                    <FieldLabel>"Comfortable"</FieldLabel>
                    <FieldDescription>"More space between elements."</FieldDescription>
                </FieldContent>
            </Field>
            <Field orientation="horizontal">
                <Radio value="compact" />
                <FieldContent>
                    <FieldLabel>"Compact"</FieldLabel>
                    <FieldDescription>"Minimal spacing for dense layouts."</FieldDescription>
                </FieldContent>
            </Field>
        </RadioGroup>
    }
}
