use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputDisabledExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs" disabled=true>
            <Input input_type="email" placeholder="Email">
                "Email"
            </Input>
            <FieldDescription>"This field is currently disabled."</FieldDescription>
        </Field>
    }
}
