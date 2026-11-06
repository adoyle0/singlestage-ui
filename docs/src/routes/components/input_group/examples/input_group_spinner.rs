use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupSpinnerExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <Input placeholder="Searching..." />
                <InputGroupAddon align="inline-end">
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Processing..." />
                <InputGroupAddon>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Saving changes..." />
                <InputGroupAddon align="inline-end">
                    <InputGroupText>"Saving..."</InputGroupText>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Refreshing data..." />
                <InputGroupAddon>
                    <Spinner>{icon!(icondata::LuLoader)}</Spinner>
                </InputGroupAddon>
                <InputGroupAddon align="inline-end">
                    <InputGroupText class="text-muted-foreground">"Please wait..."</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
