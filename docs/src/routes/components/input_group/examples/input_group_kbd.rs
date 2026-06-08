use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupKbdExample() -> impl IntoView {
    view! {
        <InputGroup class="max-w-sm">
            <Input placeholder="Search..." />
            <InputGroupAddon>
                {icon!(icondata::LuSearch, class="text-muted-foreground")}
            </InputGroupAddon>
            <InputGroupAddon align="inline-end">
                <Kbd>"⌘K"</Kbd>
            </InputGroupAddon>
        </InputGroup>
    }
}
