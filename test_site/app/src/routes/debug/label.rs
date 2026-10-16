use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugLabel() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Labels"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Labels should all be visually consistent"</li>
            <li>"label and input ids should match"</li>
        </ul>

        <h2>"Solo:"</h2>
        <Radio>"label"</Radio>
        <Checkbox>"label"</Checkbox>
        <Switch>"label"</Switch>
        <Input>"label"</Input>
        <Textarea>"label"</Textarea>

        <h2>"Field:"</h2>
        <FieldSet>
            <Field>
                <Radio>"label"</Radio>
            </Field>
            <Field>
                <Checkbox>"label"</Checkbox>
            </Field>
            <Field>
                <Switch>"label"</Switch>
            </Field>
            <Field>
                <Input>"label"</Input>
            </Field>
            <Field>
                <Textarea>"label"</Textarea>
            </Field>
        </FieldSet>
    }
}
