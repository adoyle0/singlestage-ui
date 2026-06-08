use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputExample() -> impl IntoView {
    view! {
        <FieldSet>
            <Field>
                <Input input_type="color" default="#F00">
                    "Color:"
                </Input>
            </Field>

            <Field>
                <Input input_type="date">"Date:"</Input>
            </Field>

            <Field>
                <Input input_type="datetime-local">"Datetime:"</Input>
            </Field>

            <Field>
                <Input input_type="email">"Email:"</Input>
            </Field>

            <Field>
                <Input input_type="file">"File:"</Input>
            </Field>

            <Field>
                <Input input_type="hidden" name="John Cena" />
            </Field>

            <Field>
                <Input input_type="month">"Month:"</Input>
            </Field>

            <Field>
                <Input input_type="number">"Number:"</Input>
            </Field>

            <Field>
                <Input input_type="password">"Password:"</Input>
            </Field>

            <Field>
                <Input input_type="search">"Search:"</Input>
            </Field>

            <Field>
                <Input input_type="tel">"Phone Number:"</Input>
            </Field>

            <Field disabled=true>
                <Input input_type="text">"Disabled:"</Input>
            </Field>

            <Field invalid=true>
                <Input input_type="text">"Invalid:"</Input>
            </Field>

            <Field>
                <Input input_type="time">"Time:"</Input>
            </Field>

            <Field>
                <Input input_type="url">"URL:"</Input>
            </Field>

            <Field>
                <Input input_type="week">"Week:"</Input>
            </Field>
        </FieldSet>
    }
}
