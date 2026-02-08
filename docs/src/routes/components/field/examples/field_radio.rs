use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldRadioExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldLegend variant="label">"Subscription Plan"</FieldLegend>
            <FieldDescription>
                "Yearly and lifetime plans offer significant savings."
            </FieldDescription>
            <RadioGroup value="monthly">
                <Field orientation="horizontal">
                    <Radio value="monthly" id="plan-monthly" />
                    <Label label_for="plan-monthly" class="font-normal">
                        "Monthly ($9.99/month)"
                    </Label>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="yearly" id="plan-yearly" />
                    <Label label_for="plan-yearly" class="font-normal">
                        "Yearly ($99.99/year)"
                    </Label>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="lifetime" id="plan-lifetime" />
                    <Label label_for="plan-lifetime" class="font-normal">
                        "Lifetime ($299.99)"
                    </Label>
                </Field>
            </RadioGroup>
        </FieldSet>
    }
}
