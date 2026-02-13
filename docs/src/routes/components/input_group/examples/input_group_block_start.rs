use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupBlockStartExample() -> impl IntoView {
    view! {
        <FieldGroup class="max-w-sm">
            <Field>
                <Label label_for="block-start-input">"Input"</Label>
                <InputGroup class="h-auto">
                    <Input id="block-start-input" placeholder="Enter your name" />
                    <InputGroupAddon align="block-start">
                        <InputGroupText>"Full Name"</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>
                <FieldDescription>"Header positioned above the input."</FieldDescription>
            </Field>
            <Field>
                <Label label_for="block-start-textarea">"Textarea"</Label>
                <InputGroup>
                    <Textarea
                        id="block-start-textarea"
                        placeholder="console.log('Hello, world!');"
                        class="font-mono text-sm"
                    />
                    <InputGroupAddon align="block-start">
                        {icon!(icondata::LuFileCode, class="text-muted-foreground")}
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
