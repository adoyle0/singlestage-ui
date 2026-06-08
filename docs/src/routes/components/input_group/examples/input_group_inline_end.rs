use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupInlineEndExample() -> impl IntoView {
    view! {
        <Field class="max-w-sm">
            <Label label_for="inline-end-input">"Input"</Label>
            <InputGroup>
                <Input id="inline-end-input" input_type="password" placeholder="Enter password" />
                <InputGroupAddon align="inline-end">{icon!(icondata::LuEyeOff)}</InputGroupAddon>
            </InputGroup>
            <FieldDescription>"Icon positioned at the end."</FieldDescription>
        </Field>
    }
}
