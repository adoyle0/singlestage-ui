use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverFormExample() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button variant="outline">"Open Popover"</Button>
            </PopoverTrigger>
            <PopoverContent class="w-64" align="start">
                <PopoverHeader>
                    <PopoverTitle>"Dimensions"</PopoverTitle>
                    <PopoverDescription>"Set the dimensions for the layer."</PopoverDescription>
                </PopoverHeader>
                <FieldGroup class="gap-4">
                    <Field orientation="horizontal">
                        <FieldLabel label_for="width" class="w-1/2">
                            "Width"
                        </FieldLabel>
                        <Input id="width" value="100%" />
                    </Field>
                    <Field orientation="horizontal">
                        <FieldLabel label_for="height" class="w-1/2">
                            "Height"
                        </FieldLabel>
                        <Input id="height" value="25px" />
                    </Field>
                </FieldGroup>
            </PopoverContent>
        </Popover>
    }
}
