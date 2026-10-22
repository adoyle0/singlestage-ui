use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSetExample() -> impl IntoView {
    view! {
        <FieldSet class="max-w-sm">
            <FieldLegend>"Address Information"</FieldLegend>
            <FieldDescription>"We need your address to deliver your order."</FieldDescription>
            <FieldGroup>
                <Field>
                    <Input placeholder="123 Main St">"Street Address"</Input>
                </Field>
                <FieldGroup class="grid grid-cols-2">
                    <Field>
                        <Input placeholder="New York">"City"</Input>
                    </Field>
                    <Field>
                        <Input placeholder="90502">"Postal Code"</Input>
                    </Field>
                </FieldGroup>
            </FieldGroup>
        </FieldSet>
    }
}
