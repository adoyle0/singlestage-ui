use leptos::prelude::*;
use singlestage::{Input, popover::*};

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <Popover id="demo-popover">
            <PopoverTrigger variant="outline">"Open popover"</PopoverTrigger>
            <PopoverContent class="w-80">
                <div class="grid gap-4">
                    <header class="grid gap-1.5">
                        <h4 class="leading-none font-medium">"Dimensions"</h4>
                        <p class="text-(--muted-foreground) text-sm">
                            "Set the dimensions for the layer."
                        </p>
                    </header>
                    <form class="form grid gap-2">
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Input value="100%" class="col-span-2 h-8" autofocus=true>
                                "Width"
                            </Input>
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Input value="300px" class="col-span-2 h-8">
                                "Max. width"
                            </Input>
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Input value="25px" class="col-span-2 h-8">
                                "Height"
                            </Input>
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Input value="none" class="col-span-2 h-8">
                                "Max. height"
                            </Input>
                        </div>
                    </form>
                </div>
            </PopoverContent>
        </Popover>
    }
}
