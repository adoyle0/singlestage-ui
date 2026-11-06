use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputFileExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <Input input_type="file">"Picture"</Input>
            <FieldDescription>"Select a picture to upload."</FieldDescription>
        </Field>
    }
}
