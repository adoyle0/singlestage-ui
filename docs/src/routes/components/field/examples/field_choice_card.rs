use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldChoiceCardExample() -> impl IntoView {
    let selected_environment = RwSignal::new("kubernetes".to_string());

    view! {
        <FieldSet class="max-w-xs">
            <FieldLegend variant="label">"Compute Environment"</FieldLegend>
            <FieldDescription>"Select the compute environment for your cluster."</FieldDescription>
            <FieldGroup>
                <FieldRadioGroup value=selected_environment>
                    <FieldLabel>
                        <Field orientation="horizontal">
                            <FieldContent>
                                <FieldTitle>"Kubernetes"</FieldTitle>
                                <FieldDescription>
                                    "Run GPU workloads on a K8s cluster."
                                </FieldDescription>
                            </FieldContent>
                            <Radio value="kubernetes" />
                        </Field>
                    </FieldLabel>
                    <FieldLabel>
                        <Field orientation="horizontal">
                            <FieldContent>
                                <FieldTitle>"Virtual Machine"</FieldTitle>
                                <FieldDescription>
                                    "Access a cluster to run GPU workloads."
                                </FieldDescription>
                            </FieldContent>
                            <Radio value="vm" />
                        </Field>
                    </FieldLabel>
                </FieldRadioGroup>
            </FieldGroup>
        </FieldSet>
    }
}
