use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <Popover>
            <Trigger>
                <Button variant="outline">"Open popover"</Button>
            </Trigger>
            <PopoverContent class="w-80">
                <form>
                    <FieldSet class="gap-4">
                        <FieldLegend variant="label">"Dimensions"</FieldLegend>
                        <FieldDescription>"Set the dimensions for the layer."</FieldDescription>
                        <FieldGroup class="gap-2 [&_input]:max-w-48 [&_input]:h-8">
                            <Field orientation="horizontal">
                                <Label>"Width"</Label>
                                <Input value="100%" autofocus=true />
                            </Field>
                            <Field orientation="horizontal">
                                <Label>"Max. width"</Label>
                                <Input value="300px" />
                            </Field>
                            <Field orientation="horizontal">
                                <Label>"Height"</Label>
                                <Input value="25px" />
                            </Field>
                            <Field orientation="horizontal">
                                <Label>"Max. height"</Label>
                                <Input value="none" />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </form>
            </PopoverContent>
        </Popover>
    }
}
