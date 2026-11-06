use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputButtonGroupExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <FieldLabel>"Search"</FieldLabel>
            <ButtonGroup>
                <Input placeholder="Type to search..." />
                <Button variant="outline">"Search"</Button>
            </ButtonGroup>
        </Field>
    }
}
