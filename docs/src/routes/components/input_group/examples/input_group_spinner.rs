use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupSpinnerExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup disabled=true>
                <InputGroupInput placeholder="Searching..." disabled=true />
                <InputGroupAddon align="inline-end">
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup disabled=true>
                <InputGroupInput placeholder="Processing..." disabled=true />
                <InputGroupAddon>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup disabled=true>
                <InputGroupInput placeholder="Saving changes..." disabled=true />
                <InputGroupAddon align="inline-end">
                    <InputGroupText>"Saving..."</InputGroupText>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
            <InputGroup disabled=true>
                <InputGroupInput placeholder="Refreshing data..." disabled=true />
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
