use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverAnatomy() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button />
            </PopoverTrigger>
            <PopoverContent>
                <PopoverHeader>
                    <PopoverTitle />
                    <PopoverDescription />
                </PopoverHeader>
            </PopoverContent>
        </Popover>
    }
}
