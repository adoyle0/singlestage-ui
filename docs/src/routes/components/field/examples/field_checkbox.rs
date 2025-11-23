use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldCheckboxExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldGroup>
                <FieldSet>
                    <FieldLegend variant="label">"Show these items on the desktop"</FieldLegend>
                    <FieldDescription>
                        "Select the items you want to show on the desktop."
                    </FieldDescription>
                    <FieldGroup>
                        <CheckboxGroup>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"Hard disks"</Checkbox>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"External disks"</Checkbox>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"CDs, DVDs, and iPods"</Checkbox>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox class="font-normal">"Connected servers"</Checkbox>
                            </Field>
                        </CheckboxGroup>
                    </FieldGroup>
                </FieldSet>
                <FieldSeparator />
                <Field orientation="horizontal">
                    <Checkbox checked=true />
                    <FieldLabel>
                        <FieldContent>
                            <FieldTitle>"Sync Desktop & Documents folders"</FieldTitle>
                            <FieldDescription>
                                "Your Desktop & Documents folders are being synced with iCloud
                                Drive. You can access them from other devices."
                            </FieldDescription>
                        </FieldContent>
                    </FieldLabel>
                </Field>
            </FieldGroup>
        </div>
    }
}
