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
                    <Label />
                    <FieldError />
                    <Input />
                    <FieldDescription />
                </Field>
            </FieldGroup>
            <Separator />
            <FieldGroup>
                <Field>
                    <Label>
                        <FieldContent>
                            <FieldTitle />
                            <FieldDescription />
                        </FieldContent>
                    </Label>
                    <Checkbox />
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
