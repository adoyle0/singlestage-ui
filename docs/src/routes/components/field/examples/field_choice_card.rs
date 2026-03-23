use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldChoiceCardExample() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-xs">
            <FieldSet>
                <FieldLegend variant="label">"Compute Environment"</FieldLegend>
                <FieldDescription>
                    "Select the compute environment for your cluster."
                </FieldDescription>
                <RadioGroup value="kubernetes">
                    <FieldLabel label_for="kubernetes-r2h">
                        <Field orientation="horizontal">
                            <FieldContent>
                                <FieldTitle>"Kubernetes"</FieldTitle>
                                <FieldDescription>
                                    "Run GPU workloads on a K8s cluster."
                                </FieldDescription>
                            </FieldContent>
                            <Radio value="kubernetes" id="kubernetes-r2h" />
                        </Field>
                    </FieldLabel>
                    <FieldLabel label_for="vm-z4k">
                        <Field orientation="horizontal">
                            <FieldContent>
                                <FieldTitle>"Virtual Machine"</FieldTitle>
                                <FieldDescription>
                                    "Access a cluster to run GPU workloads."
                                </FieldDescription>
                            </FieldContent>
                            <Radio value="vm" id="vm-z4k" />
                        </Field>
                    </FieldLabel>
                </RadioGroup>
            </FieldSet>
        </FieldGroup>
    }
}
