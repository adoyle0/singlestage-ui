use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldCheckboxExample() -> impl IntoView {
    let items_to_show = RwSignal::new(vec!["hard_disks".to_string()]);
    let sync_folders = RwSignal::new(true);

    view! {
        <FieldSet class="max-w-xs">
            <FieldLegend variant="label">"Show these items on the desktop"</FieldLegend>
            <FieldDescription>"Select the items you want to show on the desktop."</FieldDescription>
            <FieldGroup>
                <FieldCheckboxGroup value=items_to_show>
                    <Field orientation="horizontal">
                        <Checkbox value="hard_disks">"Hard disks"</Checkbox>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox value="external_disks">"External disks"</Checkbox>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox value="cd_dvd_ipod">"CDs, DVDs, and iPods"</Checkbox>
                    </Field>
                    <Field orientation="horizontal">
                        <Checkbox value="connected_servers">"Connected servers"</Checkbox>
                    </Field>
                </FieldCheckboxGroup>
            </FieldGroup>
            <FieldSeparator />
            <Field orientation="horizontal">
                <Checkbox checked=sync_folders />
                <FieldContent>
                    <FieldLabel>"Sync Desktop & Documents folders"</FieldLabel>
                    <FieldDescription>
                        "Your Desktop & Documents folders are being synced with iCloud Drive.
                        You can access them from other devices."
                    </FieldDescription>
                </FieldContent>
            </Field>
        </FieldSet>
    }
}
