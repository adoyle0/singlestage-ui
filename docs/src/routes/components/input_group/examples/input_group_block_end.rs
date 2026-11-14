use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupBlockEndExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field>
                <FieldLabel>"Input"</FieldLabel>
                <InputGroup class="h-auto">
                    <Input placeholder="Enter amount" />
                    <InputGroupAddon align="block-end">
                        <InputGroupText>"USD"</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Footer positioned below the input."</FieldDescription>
            </Field>
            <Field>
                <FieldLabel>"Textarea"</FieldLabel>
                <InputGroup>
                    <Textarea placeholder="Write a comment..." />
                    <InputGroupAddon align="block-end">
                        <InputGroupText>"0/280"</InputGroupText>
                        <Button variant="default" size="sm" class="ml-auto">
                            "Post"
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Footer positioned below the textarea."</FieldDescription>
            </Field>
        </FieldGroup>
    }
}
