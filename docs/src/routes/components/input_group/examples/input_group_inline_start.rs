use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupInlineStartExample() -> impl IntoView {
    view! {
        <Field class="max-w-sm">
            <Label label_for="inline-start-input">"Input"</Label>
            <InputGroup>
                <Input id="inline-start-input" placeholder="Search..." />
                <InputGroupAddon align="inline-start">
                    {icon!(icondata::LuSearch, class="text-muted-foreground")}
                </InputGroupAddon>
            </InputGroup>
            <FieldDescription>"Icon positioned at the start."</FieldDescription>
        </Field>
    }
}
