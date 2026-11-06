use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputFormExample() -> impl IntoView {
    view! {
        <form class="w-full max-w-sm">
            <FieldGroup>
                <Field>
                    <Input input_type="text" placeholder="Evil Rabbit">
                        "Name"
                    </Input>
                </Field>
                <Field>
                    <Input input_type="email" placeholder="john@example.com">
                        "Email"
                    </Input>
                    <FieldDescription>"We'll never share your email with anyone."</FieldDescription>
                </Field>
                <div class="grid grid-cols-2 gap-4">
                    <Field>
                        <Input input_type="tel" placeholder="+1 (555) 123-4567">
                            "Phone"
                        </Input>
                    </Field>
                    <Field>
                        <FieldLabel>Country</FieldLabel>
                        <Select default="us">
                            <SelectOption value="us">"United States"</SelectOption>
                            <SelectOption value="uk">"United Kingdom"</SelectOption>
                            <SelectOption value="ca">"Canada"</SelectOption>
                        </Select>
                    </Field>
                </div>
                <Field>
                    <Input input_type="text" placeholder="123 Main St">
                        "Address"
                    </Input>
                </Field>
                <Field orientation="horizontal">
                    <Button button_type="button" variant="outline">
                        "Cancel"
                    </Button>
                    <Button>"Submit"</Button>
                </Field>
            </FieldGroup>
        </form>
    }
}
