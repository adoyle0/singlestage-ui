use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerInputGroupExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            <InputGroup>
                <InputGroupInput placeholder="Send a message..." disabled=true />
                <InputGroupAddon align="inline-end">
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupTextarea placeholder="Send a message..." disabled=true />
                <InputGroupAddon align="block-end">
                    <Spinner />
                    "Validating..."
                    <InputGroupButton class="ml-auto" variant="default">
                        {icon!(icondata::LuArrowUp)}
                        <span class="sr-only">"Send"</span>
                    </InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
