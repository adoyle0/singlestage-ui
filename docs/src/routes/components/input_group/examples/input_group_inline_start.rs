use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupInlineStartExample() -> impl IntoView {
    view! {
        <Field class="max-w-sm">
            <FieldLabel>"Input"</FieldLabel>
            <InputGroup>
                <Input placeholder="Search..." />
                <InputGroupAddon align="inline-start">{icon!(icondata::LuSearch)}</InputGroupAddon>
            </InputGroup>
            <FieldDescription>"Icon positioned at the start."</FieldDescription>
        </Field>
    }
}
