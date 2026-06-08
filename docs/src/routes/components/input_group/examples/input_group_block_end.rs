use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupBlockEndExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field>
                <FieldLabel label_for="block-end-input">"Input"</FieldLabel>
                <InputGroup class="h-auto">
                    <Input id="block-end-input" placeholder="Enter amount" />
                    <InputGroupAddon align="block-end">
                        <InputGroupText>"USD"</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Footer positioned below the input."</FieldDescription>
            </Field>
            <Field>
                <FieldLabel label_for="block-end-textarea">"Textarea"</FieldLabel>
                <InputGroup>
                    <Textarea id="block-end-textarea" placeholder="Write a comment..." />
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
