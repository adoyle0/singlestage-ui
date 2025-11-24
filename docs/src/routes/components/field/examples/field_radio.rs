use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldRadioExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldTitle>"Subscription Plan"</FieldTitle>
                <FieldDescription>
                    "Yearly and lifetime plans offer significant savings."
                </FieldDescription>
                <FieldGroup>
                    <RadioGroup default="monthly">
                        <Field orientation="horizontal">
                            <Radio class="font-normal" value="monthly">
                                "Monthly ($9.99/month)"
                            </Radio>
                        </Field>
                        <Field orientation="horizontal">
                            <Radio class="font-normal" value="yearly">
                                "Yearly ($99.99/year)"
                            </Radio>
                        </Field>
                        <Field orientation="horizontal">
                            <Radio class="font-normal" value="lifetime">
                                "Lifetime ($299.99)"
                            </Radio>
                        </Field>
                    </RadioGroup>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
