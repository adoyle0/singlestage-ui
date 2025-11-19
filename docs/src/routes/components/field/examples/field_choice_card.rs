use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldChoiceCardExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldGroup>
                <FieldSet>
                    <FieldLabel label_for="compute-environment-p8w">
                        "Compute Environment"
                    </FieldLabel>
                    <FieldDescription>
                        "Select the compute environment for your cluster."
                    </FieldDescription>
                    <RadioGroup default="kubernetes">
                        <FieldLabel label_for="kubernetes-r2h">
                            <Field orientation="horizontal">
                                <FieldContent>
                                    <FieldTitle>"Kubernetes"</FieldTitle>
                                    <FieldDescription>
                                        "Run GPU workloads on a K8s configured cluster."
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
                                        "Access a VM configured cluster to run GPU workloads."
                                    </FieldDescription>
                                </FieldContent>
                                <Radio value="vm" id="vm-z4k" />
                            </Field>
                        </FieldLabel>
                    </RadioGroup>
                </FieldSet>
            </FieldGroup>
        </div>
    }
}
