use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldResponsiveLayoutExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-4xl">
            <form>
                <FieldSet>
                    <FieldLegend>"Profile"</FieldLegend>
                    <FieldDescription>"Fill in your profile information."</FieldDescription>
                    <FieldSeparator />
                    <FieldGroup>
                        <Field orientation="responsive">
                            <FieldContent>
                                <FieldLabel>"Name"</FieldLabel>
                                <FieldDescription>
                                    "Provide your full name for identification"
                                </FieldDescription>
                            </FieldContent>
                            <Input placeholder="Evil Rabbit" />
                        </Field>
                        <FieldSeparator />
                        <Field orientation="responsive">
                            <FieldContent>
                                <FieldLabel>"Message"</FieldLabel>
                                <FieldDescription>
                                    "You can write your message here. Keep it short, preferably
                                    under 100 characters."
                                </FieldDescription>
                            </FieldContent>
                            <Textarea
                                placeholder="Hello, world!"
                                class="min-h-[100px] resize-none sm:min-w-[300px]"
                            />
                        </Field>
                        <FieldSeparator />
                        <Field orientation="responsive">
                            <Button button_type="submit">"Submit"</Button>
                            <Button button_type="button" variant="outline">
                                "Cancel"
                            </Button>
                        </Field>
                    </FieldGroup>
                </FieldSet>
            </form>
        </div>
    }
}
