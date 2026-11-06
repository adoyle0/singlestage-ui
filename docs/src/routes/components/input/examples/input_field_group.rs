use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputFieldGroupExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-xs">
            <Field>
                <Input placeholder="Jordan Lee">"Name"</Input>
            </Field>
            <Field>
                <Input input_type="email" placeholder="name@example.com">
                    "Email"
                </Input>
                <FieldDescription>"We'll send updates to this address."</FieldDescription>
            </Field>
            <Field orientation="horizontal">
                <Button button_type="reset" variant="outline">
                    "Reset"
                </Button>
                <Button>"Submit"</Button>
            </Field>
        </FieldGroup>
    }
}
