use leptos::*;
use singlestage::field::*;

#[component]
pub fn FieldAnatomy() -> impl IntoView {
    view! {
        <FieldSet>
            <FieldLegend />
            <FieldGroup>
                <Field>
                    <FieldLabel>
                        <FieldContent>
                            <FieldTitle />
                            <FieldDescription />
                        </FieldContent>
                    </FieldLabel>
                </Field>
            </FieldGroup>
            <FieldSeparator />
        </FieldSet>
    }
}
