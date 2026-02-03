use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxGroupExample() -> impl IntoView {
    view! {
        <FieldSet>
            <FieldLegend variant="label">"Show these items on the desktop:"</FieldLegend>
            <FieldDescription>"Select the items you want to show on the desktop."</FieldDescription>
            <FieldGroup class="gap-3">
                <Field orientation="horizontal">
                    <Checkbox checked=true>"Hard disks"</Checkbox>
                </Field>
                <Field orientation="horizontal">
                    <Checkbox checked=true>"External disks"</Checkbox>
                </Field>
                <Field orientation="horizontal">
                    <Checkbox>"CDs, DVDs, and iPods"</Checkbox>
                </Field>
                <Field orientation="horizontal">
                    <Checkbox>"Connected servers"</Checkbox>
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
