use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CheckboxGroupExample() -> impl IntoView {
    view! {
        <FieldSet>
            <FieldLegend variant="label">"Show these items on the desktop:"</FieldLegend>
            <FieldDescription>"Select the items you want to show on the desktop."</FieldDescription>
            <CheckboxGroup class="gap-3">
                <Field orientation="horizontal">
                    <Checkbox value="hard_disks" checked=true>
                        "Hard disks"
                    </Checkbox>
                </Field>

                <Field orientation="horizontal">
                    <Checkbox value="external_disks" checked=true>
                        "External disks"
                    </Checkbox>
                </Field>

                <Field orientation="horizontal">
                    <Checkbox value="cds">"CDs, DVDs, and iPods"</Checkbox>
                </Field>

                <Field orientation="horizontal">
                    <Checkbox value="servers">"Connected servers"</Checkbox>
                </Field>
            </CheckboxGroup>
        </FieldSet>
    }
}
