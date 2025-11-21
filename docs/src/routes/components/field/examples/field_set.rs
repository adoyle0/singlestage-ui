use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSetExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md space-y-6">
            <FieldSet>
                <FieldLegend>"Address Information"</FieldLegend>
                <FieldDescription>"We need your address to deliver your order."</FieldDescription>
                <FieldGroup>
                    <Field>
                        <FieldLabel label_for="street">"Street Address"</FieldLabel>
                        <Input id="street" input_type="text" placeholder="123 Main St" />
                    </Field>
                    <div class="grid grid-cols-2 gap-4">
                        <Field>
                            <FieldLabel label_for="city">"City"</FieldLabel>
                            <Input id="city" input_type="text" placeholder="Torrance" />
                        </Field>
                        <Field>
                            <FieldLabel label_for="zip">"Postal Code"</FieldLabel>
                            <Input id="zip" input_type="text" placeholder="90502" />
                        </Field>
                    </div>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
