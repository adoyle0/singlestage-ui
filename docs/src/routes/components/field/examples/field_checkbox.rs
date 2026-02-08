use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldCheckboxExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-xs">
            <FieldSet>
                <FieldLegend variant="label">"Show these items on the desktop"</FieldLegend>
                <FieldDescription>
                    "Select the items you want to show on the desktop."
                </FieldDescription>
                <FieldGroup class="gap-3">
                    <Field orientation="horizontal">
                        <Checkbox id="finder-pref-9k2-hard-disks-ljj" checked=true />
                        <Label label_for="finder-pref-9k2-hard-disks-ljj" class="font-normal">
                            "Hard disks"
                        </Label>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox id="finder-pref-9k2-external-disks-1yg" />
                        <Label label_for="finder-pref-9k2-external-disks-1yg" class="font-normal">
                            "External disks"
                        </Label>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox id="finder-pref-9k2-cds-dvds-fzt" />
                        <Label label_for="finder-pref-9k2-cds-dvds-fzt" class="font-normal">
                            "CDs, DVDs, and iPods"
                        </Label>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox id="finder-pref-9k2-connected-servers-6l2" />
                        <Label
                            label_for="finder-pref-9k2-connected-servers-6l2"
                            class="font-normal"
                        >
                            "Connected servers"
                        </Label>
                    </Field>
                </FieldGroup>
            </FieldSet>
            <Separator />
            <Field orientation="horizontal">
                <Checkbox id="finder-pref-9k2-sync-folders-nep" checked=true />
                <FieldContent>
                    <Label label_for="finder-pref-9k2-sync-folders-nep">
                        "Sync Desktop & Documents folders"
                    </Label>
                    <FieldDescription>
                        "Your Desktop & Documents folders are being synced with iCloud Drive.
                        You can access them from other devices."
                    </FieldDescription>
                </FieldContent>
            </Field>
        </FieldGroup>
    }
}
