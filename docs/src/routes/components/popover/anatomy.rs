use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverAnatomy() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button />
            </PopoverTrigger>
            <PopoverContent />
        </Popover>
    }
}
