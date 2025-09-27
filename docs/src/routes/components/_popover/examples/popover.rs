use leptos::prelude::*;
use singlestage::{popover::*, Button, Input, Label};

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <Popover id="demo-popover">
            <PopoverTrigger slot>
                <Button
                    id="demo-popover-trigger"
                    button_type="button"
                    attr:aria-expanded="false"
                    attr:aria-controls="demo-popover-popover"
                    variant="outline"
                >
                    "Open popover"
                </Button>
            </PopoverTrigger>
            <PopoverContent id="demo-popover-popover" class="w-80">
                <div class="grid gap-4">
                    <header class="grid gap-1.5">
                        <h4 class="leading-none font-medium">"Dimensions"</h4>
                        <p class="text-muted-foreground text-sm">
                            "Set the dimensions for the layer."
                        </p>
                    </header>
                    <form class="form grid gap-2">
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Label label_for="demo-popover-width">"Width"</Label>
                            <Input
                                id="demo-popover-width"
                                value="100%"
                                class="col-span-2 h-8"
                                autofocus=true
                            />
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Label label_for="demo-popover-max-width">"Max. width"</Label>
                            <Input
                                id="demo-popover-max-width"
                                value="300px"
                                class="col-span-2 h-8"
                            />
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Label label_for="demo-popover-height">"Height"</Label>
                            <Input id="demo-popover-height" value="25px" class="col-span-2 h-8" />
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <Label label_for="demo-popover-max-height">"Max. height"</Label>
                            <Input
                                id="demo-popover-max-height"
                                value="none"
                                class="col-span-2 h-8"
                            />
                        </div>
                    </form>
                </div>
            </PopoverContent>
        </Popover>
    }
}
