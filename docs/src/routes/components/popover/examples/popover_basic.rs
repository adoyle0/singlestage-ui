use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverBasicExample() -> impl IntoView {
    view! {
        <Popover>
            <Trigger>
                <Button variant="outline">"Open Popover"</Button>
            </Trigger>
            <PopoverContent align="start">
                <PopoverHeader>
                    <PopoverTitle>"Dimensions"</PopoverTitle>
                    <PopoverDescription>"Set the dimensions for the layer."</PopoverDescription>
                </PopoverHeader>
            </PopoverContent>
        </Popover>
    }
}
