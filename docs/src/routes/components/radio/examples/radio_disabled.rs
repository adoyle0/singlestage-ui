use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn RadioDisabledExample() -> impl IntoView {
    view! {
        <RadioGroup default="option2" class="w-fit">
            <Field orientation="horizontal" disabled=true>
                <Radio value="option1" id="disabled-1" disabled=true />
                <Label label_for="disabled-1" class="font-normal">
                    "Disabled"
                </Label>
            </Field>
            <Field orientation="horizontal">
                <Radio value="option2" id="disabled-2" />
                <Label label_for="disabled-2" class="font-normal">
                    "Option 2"
                </Label>
            </Field>
            <Field orientation="horizontal">
                <Radio value="option3" id="disabled-3" />
                <Label label_for="disabled-3" class="font-normal">
                    "Option 3"
                </Label>
            </Field>
        </RadioGroup>
    }
}
