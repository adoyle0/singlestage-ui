use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerInputGroupExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            <InputGroup>
                <Input placeholder="Send a message..." disabled=true />
                <InputGroupAddon align="inline-end">
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Textarea placeholder="Send a message..." disabled=true />
                <InputGroupAddon align="block-end">
                    <Spinner />
                    "Validating..."
                    <Button class="ml-auto" variant="default">
                        {icon!(icondata::LuArrowUp)}
                        <span class="sr-only">"Send"</span>
                    </Button>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
