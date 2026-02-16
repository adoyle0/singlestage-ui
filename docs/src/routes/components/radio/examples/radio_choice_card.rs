use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioChoiceCardExample() -> impl IntoView {
    view! {
        <RadioGroup default="plus" class="max-w-sm">
            <Label label_for="plus-plan">
                <Field orientation="horizontal">
                    <FieldContent>
                        <FieldTitle>"Plus"</FieldTitle>
                        <FieldDescription>"For individuals and small teams."</FieldDescription>
                    </FieldContent>
                    <Radio value="plus" id="plus-plan" />
                </Field>
            </Label>
            <Label label_for="pro-plan">
                <Field orientation="horizontal">
                    <FieldContent>
                        <FieldTitle>"Pro"</FieldTitle>
                        <FieldDescription>"For growing businesses."</FieldDescription>
                    </FieldContent>
                    <Radio value="pro" id="pro-plan" />
                </Field>
            </Label>
            <Label label_for="enterprise-plan">
                <Field orientation="horizontal">
                    <FieldContent>
                        <FieldTitle>"Enterprise"</FieldTitle>
                        <FieldDescription>"For large teams and enterprises."</FieldDescription>
                    </FieldContent>
                    <Radio value="enterprise" id="enterprise-plan" />
                </Field>
            </Label>
        </RadioGroup>
    }
}
