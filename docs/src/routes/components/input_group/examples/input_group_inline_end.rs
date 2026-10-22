use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupInlineEndExample() -> impl IntoView {
    view! {
        <Field class="max-w-sm">
            <FieldLabel>"Input"</FieldLabel>
            <InputGroup>
                <Input input_type="password" placeholder="Enter password" />
                <InputGroupAddon align="inline-end">{icon!(icondata::LuEyeOff)}</InputGroupAddon>
            </InputGroup>
            <FieldDescription>"Icon positioned at the end."</FieldDescription>
        </Field>
    }
}
