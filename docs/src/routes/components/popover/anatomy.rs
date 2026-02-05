use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverAnatomy() -> impl IntoView {
    view! {
        <Popover>
            <Trigger>
                <Button />
            </Trigger>
            <PopoverContent>
                <PopoverHeader>
                    <PopoverTitle />
                    <PopoverDescription />
                </PopoverHeader>
            </PopoverContent>
        </Popover>
    }
}
