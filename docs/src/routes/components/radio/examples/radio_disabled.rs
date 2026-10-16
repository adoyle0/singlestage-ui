use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioDisabledExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldLegend variant="label">"Notification Preferences"</FieldLegend>
            <FieldDescription>"Choose how you want to receive notifications."</FieldDescription>
            <RadioGroup default="email" disabled=true>
                <Field orientation="horizontal" disabled=true>
                    <Radio value="email">"Email only"</Radio>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="sms" disabled=true>
                        "SMS only"
                    </Radio>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="both">"Both Email & SMS"</Radio>
                </Field>
            </RadioGroup>
        </FieldSet>
    }
}
