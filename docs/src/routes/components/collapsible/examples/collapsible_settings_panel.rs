use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CollapsibleSettingsPanelExample() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Card class="mx-auto w-full max-w-xs" size="sm">
            <CardHeader>
                <CardTitle>"Radius"</CardTitle>
                <CardDescription>"Set the corner radius of the element."</CardDescription>
            </CardHeader>
            <CardContent>
                <Collapsible open class="flex items-start gap-2">
                    <FieldGroup class="grid w-full grid-cols-2 gap-2">
                        <Field>
                            <FieldLabel class="sr-only">"Radius X"</FieldLabel>
                            <Input placeholder="0" />
                        </Field>
                        <Field>
                            <FieldLabel class="sr-only">"Radius Y"</FieldLabel>
                            <Input placeholder="0" />
                        </Field>
                        <CollapsibleContent class="col-span-full grid grid-cols-subgrid gap-2">
                            <Field>
                                <FieldLabel class="sr-only">"Radius X"</FieldLabel>
                                <Input placeholder="0" />
                            </Field>
                            <Field>
                                <FieldLabel class="sr-only">"Radius Y"</FieldLabel>
                                <Input placeholder="0" />
                            </Field>
                        </CollapsibleContent>
                    </FieldGroup>
                    <CollapsibleTrigger>
                        <Button variant="outline" size="icon">
                            <Show
                                when=move || open.get()
                                fallback=move || { icon!(icondata::LuMaximize) }
                            >
                                {icon!(icondata::LuMinimize)}
                            </Show>
                        </Button>
                    </CollapsibleTrigger>
                </Collapsible>
            </CardContent>
        </Card>
    }
}
