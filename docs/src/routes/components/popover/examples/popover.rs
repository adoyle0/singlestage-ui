use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button variant="outline">"Open popover"</Button>
            </PopoverTrigger>
            <PopoverContent class="w-80">
                <form>
                    <FieldSet class="gap-4">
                        <FieldLegend variant="label">"Dimensions"</FieldLegend>
                        <FieldDescription>"Set the dimensions for the layer."</FieldDescription>
                        <FieldGroup class="gap-2 [&_input]:max-w-48 [&_input]:h-8">
                            <Field orientation="horizontal">
                                <FieldLabel>"Width"</FieldLabel>
                                <Input value="100%" autofocus=true />
                            </Field>
                            <Field orientation="horizontal">
                                <FieldLabel>"Max. width"</FieldLabel>
                                <Input value="300px" />
                            </Field>
                            <Field orientation="horizontal">
                                <FieldLabel>"Height"</FieldLabel>
                                <Input value="25px" />
                            </Field>
                            <Field orientation="horizontal">
                                <FieldLabel>"Max. height"</FieldLabel>
                                <Input value="none" />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </form>
            </PopoverContent>
        </Popover>
    }
}
