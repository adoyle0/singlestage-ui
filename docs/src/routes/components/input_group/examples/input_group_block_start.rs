use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupBlockStartExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field>
                <FieldLabel>"Input"</FieldLabel>
                <InputGroup class="h-auto">
                    <Input placeholder="Enter your name" />
                    <InputGroupAddon align="block-start">
                        <InputGroupText>"Full Name"</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Header positioned above the input."</FieldDescription>
            </Field>
            <Field>
                <FieldLabel>"Textarea"</FieldLabel>
                <InputGroup>
                    <Textarea
                        class="font-mono text-sm"
                        placeholder="console.log('Hello, world!');"
                    />
                    <InputGroupAddon align="block-start">
                        {icon!(icondata::LuFileCode)}
                        <InputGroupText class="font-mono">"script.js"</InputGroupText>
                        <Button size="icon-xs" class="ml-auto">
                            {icon!(icondata::LuCopy)}
                            <span class="sr-only">"Copy"</span>
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Header positioned above the textarea."</FieldDescription>
            </Field>
        </FieldGroup>
    }
}
