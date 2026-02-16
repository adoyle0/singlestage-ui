use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioInvalidExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldLegend variant="label">"Notification Preferences"</FieldLegend>
            <FieldDescription>"Choose how you want to receive notifications."</FieldDescription>
            <RadioGroup default="email" invalid=true>
                <Field orientation="horizontal" invalid=true>
                    <Radio value="email" id="invalid-email" />
                    <Label label_for="invalid-email" class="font-normal">
                        "Email only"
                    </Label>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="sms" id="invalid-sms" invalid=true />
                    <Label label_for="invalid-sms" class="font-normal">
                        "SMS only"
                    </Label>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="both" id="invalid-both" />
                    <Label label_for="invalid-both" class="font-normal">
                        "Both Email & SMS"
                    </Label>
                </Field>
            </RadioGroup>
        </FieldSet>
    }
}
