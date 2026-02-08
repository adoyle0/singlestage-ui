use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSetExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-sm">
            <FieldLegend>"Address Information"</FieldLegend>
            <FieldDescription>"We need your address to deliver your order."</FieldDescription>
            <FieldGroup>
                <Field>
                    <Label label_for="street">"Street Address"</Label>
                    <Input id="street" input_type="text" placeholder="123 Main St" />
                </Field>
                <div class="grid grid-cols-2 gap-4">
                    <Field>
                        <Label label_for="city">City</Label>
                        <Input id="city" input_type="text" placeholder="New York" />
                    </Field>
                    <Field>
                        <Label label_for="zip">Postal Code</Label>
                        <Input id="zip" input_type="text" placeholder="90502" />
                    </Field>
                </div>
            </FieldGroup>
        </FieldSet>
    }
}
