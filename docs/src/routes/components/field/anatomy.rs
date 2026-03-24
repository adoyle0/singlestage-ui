use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldAnatomy() -> impl IntoView {
    view! {
        <FieldSet>
            <FieldLegend />
            <FieldDescription />
            <FieldGroup>
                <Field>
                    <FieldLabel />
                    <FieldError />
                    <Input />
                    <FieldDescription />
                </Field>
            </FieldGroup>
            <FieldSeparator />
            <FieldGroup>
                <Field>
                    <FieldLabel>
                        <FieldContent>
                            <FieldTitle />
                            <FieldDescription />
                        </FieldContent>
                    </FieldLabel>
                    <Checkbox />
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
