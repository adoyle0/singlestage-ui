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
                    <Radio value="email">"Email only"</Radio>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="sms" invalid=true>
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
