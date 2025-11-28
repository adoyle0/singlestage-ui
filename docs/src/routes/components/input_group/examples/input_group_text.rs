use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupTextExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-6">
            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>"$"</InputGroupText>
                </InputGroupAddon>
                <Input placeholder="0.00" />
                <InputGroupAddon align="inline-end">
                    <InputGroupText>"USD"</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>"https://"</InputGroupText>
                </InputGroupAddon>
                <Input placeholder="example.com" class="!pl-0.5" />
                <InputGroupAddon align="inline-end">
                    <InputGroupText>".com"</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Enter your username" />
                <InputGroupAddon align="inline-end">
                    <InputGroupText>"@company.com"</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Textarea placeholder="Enter your message" />
                <InputGroupAddon align="block-end">
                    <InputGroupText class="text-muted-foreground text-xs">
                        "120 characters left"
                    </InputGroupText>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
