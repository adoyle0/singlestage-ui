use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <Popover id="demo-popover">
            <PopoverTrigger variant="outline">"Open popover"</PopoverTrigger>
            <PopoverContent class="w-80">
                <form>
                    <FieldSet>
                        <FieldLegend>"Dimensions"</FieldLegend>
                        <FieldDescription>"Set the dimensions for the layer."</FieldDescription>
                        <FieldGroup class="[&_input]:w-48 gap-2">
                            <Field orientation="horizontal">
                                <Input value="100%" autofocus=true>
                                    "Width"
                                </Input>
                            </Field>
                            <Field orientation="horizontal">
                                <Input value="300px">"Max. width"</Input>
                            </Field>
                            <Field orientation="horizontal">
                                <Input value="25px">"Height"</Input>
                            </Field>
                            <Field orientation="horizontal">
                                <Input value="none">"Max. height"</Input>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </form>
            </PopoverContent>
        </Popover>
    }
}
