use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LabelsImplicitExample() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <Input>"Label"</Input>
            <Checkbox>"Label"</Checkbox>
            <Radio>"Label"</Radio>
            <Switch>"Label"</Switch>
            <Textarea>"Label"</Textarea>
        </div>
    }
}
