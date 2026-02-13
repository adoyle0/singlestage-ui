use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn KbdInputGroupExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-xs flex-col gap-6">
            <InputGroup>
                <Input placeholder="Search..." />
                <InputGroupAddon>{icon!(icondata::LuSearch)}</InputGroupAddon>
                <InputGroupAddon align="inline-end">
                    <Kbd>"⌘"</Kbd>
                    <Kbd>"K"</Kbd>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
