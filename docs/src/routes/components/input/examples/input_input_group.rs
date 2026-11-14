use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputInputGroupExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <FieldLabel>Website URL</FieldLabel>
            <InputGroup>
                <Input placeholder="example.com" />
                <InputGroupAddon>
                    <InputGroupText>"https://"</InputGroupText>
                </InputGroupAddon>
                <InputGroupAddon align="inline-end">{icon!(icondata::LuInfo)}</InputGroupAddon>
            </InputGroup>
        </Field>
    }
}
