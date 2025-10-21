use leptos::prelude::*;
use singlestage::popover::*;

#[component]
pub fn PopoverAnatomy() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger />
            <PopoverContent />
        </Popover>
    }
}
