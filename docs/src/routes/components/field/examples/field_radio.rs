use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldRadioExample() -> impl IntoView {
    let selected_plan = RwSignal::new("monthly".to_string());

    view! {
        <FieldSet class="max-w-xs">
            <FieldLegend variant="label">"Subscription Plan"</FieldLegend>
            <FieldDescription>
                "Yearly and lifetime plans offer significant savings."
            </FieldDescription>
            <FieldGroup>
                <FieldRadioGroup value=selected_plan>
                    <Field orientation="horizontal">
                        <Radio value="monthly">"Monthly ($9.99/month)"</Radio>
                    </Field>
                    <Field orientation="horizontal">
                        <Radio value="yearly">"Yearly ($99.99/year)"</Radio>
                    </Field>
                    <Field orientation="horizontal">
                        <Radio value="lifetime">"Lifetime ($299.99)"</Radio>
                    </Field>
                </FieldRadioGroup>
            </FieldGroup>
        </FieldSet>
    }
}
