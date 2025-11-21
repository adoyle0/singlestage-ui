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
                                <Checkbox id="finder-pref-9k2-hard-disks-ljj" />
                                <FieldLabel
                                    label_for="finder-pref-9k2-hard-disks-ljj"
                                    class="font-normal"
                                >
                                    "Hard disks"
                                </FieldLabel>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox id="finder-pref-9k2-external-disks-1yg" />
                                <FieldLabel
                                    label_for="finder-pref-9k2-external-disks-1yg"
                                    class="font-normal"
                                >
                                    "External disks"
                                </FieldLabel>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox id="finder-pref-9k2-cds-dvds-fzt" />
                                <FieldLabel
                                    label_for="finder-pref-9k2-cds-dvds-fzt"
                                    class="font-normal"
                                >
                                    "CDs, DVDs, and iPods"
                                </FieldLabel>
                            </Field>
                            <Field orientation="horizontal">
                                <Checkbox id="finder-pref-9k2-connected-servers-6l2" />
                                <FieldLabel
                                    label_for="finder-pref-9k2-connected-servers-6l2"
                                    class="font-normal"
                                >
                                    "Connected servers"
                                </FieldLabel>
                            </Field>
                        </CheckboxGroup>
                    </FieldGroup>
                </FieldSet>
                <FieldSeparator />
                <Field orientation="horizontal">
                    <Checkbox id="finder-pref-9k2-sync-folders-nep" checked=true />
                    <FieldLabel label_for="finder-pref-9k2-sync-folders-nep">
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
