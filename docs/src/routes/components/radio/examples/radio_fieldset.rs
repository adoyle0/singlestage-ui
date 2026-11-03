use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioFieldsetExample() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldLegend variant="label">"Subscription Plan"</FieldLegend>
            <FieldDescription>
                "Yearly and lifetime plans offer significant savings."
            </FieldDescription>
            <RadioGroup default="monthly">
                <Field orientation="horizontal">
                    <Radio value="monthly">"Monthly ($9.99/month)"</Radio>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="yearly">"Yearly ($99.99/year)"</Radio>
                </Field>
                <Field orientation="horizontal">
                    <Radio value="lifetime">"Lifetime ($299.99)"</Radio>
                </Field>
            </RadioGroup>
        </FieldSet>
    }
}
