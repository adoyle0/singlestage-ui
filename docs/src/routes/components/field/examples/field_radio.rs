use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldRadioExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldLabel>"Subscription Plan"</FieldLabel>
                <FieldDescription>
                    "Yearly and lifetime plans offer significant savings."
                </FieldDescription>
                <RadioGroup default="monthly">
                    <Field orientation="horizontal">
                        <Radio value="monthly" id="plan-monthly" />
                        <FieldLabel label_for="plan-monthly" class="font-normal">
                            "Monthly ($9.99/month)"
                        </FieldLabel>
                    </Field>
                    <Field orientation="horizontal">
                        <Radio value="yearly" id="plan-yearly" />
                        <FieldLabel label_for="plan-yearly" class="font-normal">
                            "Yearly ($99.99/year)"
                        </FieldLabel>
                    </Field>
                    <Field orientation="horizontal">
                        <Radio value="lifetime" id="plan-lifetime" />
                        <FieldLabel label_for="plan-lifetime" class="font-normal">
                            "Lifetime ($299.99)"
                        </FieldLabel>
                    </Field>
                </RadioGroup>
            </FieldSet>
        </div>
    }
}
