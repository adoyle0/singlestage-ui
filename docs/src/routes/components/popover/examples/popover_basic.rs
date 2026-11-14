use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverBasicExample() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button variant="outline">"Open Popover"</Button>
            </PopoverTrigger>
            <PopoverContent align="start">
                <PopoverHeader>
                    <PopoverTitle>"Dimensions"</PopoverTitle>
                    <PopoverDescription>"Set the dimensions for the layer."</PopoverDescription>
                </PopoverHeader>
            </PopoverContent>
        </Popover>
    }
}
