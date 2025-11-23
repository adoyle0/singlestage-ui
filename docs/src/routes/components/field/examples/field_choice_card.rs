use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldChoiceCardExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldLegend variant="label">"Compute Environment"</FieldLegend>
                <FieldDescription>
                    "Select the compute environment for your cluster."
                </FieldDescription>
                <FieldGroup>
                    <RadioGroup default="kubernetes">
                        <Field orientation="horizontal" variant="button">
                            <FieldLabel>
                                <FieldContent>
                                    <FieldTitle>"Kubernetes"</FieldTitle>
                                    <FieldDescription>
                                        "Run GPU workloads on a K8s configured cluster."
                                    </FieldDescription>
                                </FieldContent>
                            </FieldLabel>
                            <Radio value="kubernetes" />
                        </Field>
                        <Field orientation="horizontal" variant="button">
                            <FieldLabel>
                                <FieldContent>
                                    <FieldTitle>"Virtual Machine"</FieldTitle>
                                    <FieldDescription>
                                        "Access a VM configured cluster to run GPU workloads."
                                    </FieldDescription>
                                </FieldContent>
                            </FieldLabel>
                            <Radio value="vm" />
                        </Field>
                    </RadioGroup>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
