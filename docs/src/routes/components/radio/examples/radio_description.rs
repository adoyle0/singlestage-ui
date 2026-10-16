use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioDescriptionExample() -> impl IntoView {
    view! {
        <RadioGroup default="comfortable" class="w-fit">
            <Field orientation="horizontal">
                <Radio value="default" id="desc-r1" />
                <FieldContent>
                    <Label label_for="desc-r1">"Default"</Label>
                    <FieldDescription>"Standard spacing for most use cases."</FieldDescription>
                </FieldContent>
            </Field>
            <Field orientation="horizontal">
                <Radio value="comfortable" id="desc-r2" />
                <FieldContent>
                    <Label label_for="desc-r2">"Comfortable"</Label>
                    <FieldDescription>"More space between elements."</FieldDescription>
                </FieldContent>
            </Field>
            <Field orientation="horizontal">
                <Radio value="compact" id="desc-r3" />
                <FieldContent>
                    <Label label_for="desc-r3">"Compact"</Label>
                    <FieldDescription>"Minimal spacing for dense layouts."</FieldDescription>
                </FieldContent>
            </Field>
        </RadioGroup>
    }
}
