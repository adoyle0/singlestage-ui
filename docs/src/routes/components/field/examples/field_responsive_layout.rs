use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldResponsiveLayoutExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-lg">
            <form>
                <FieldSet>
                    <FieldLegend>"Profile"</FieldLegend>
                    <FieldDescription>"Fill in your profile information."</FieldDescription>
                    <FieldGroup>
                        <Field orientation="responsive">
                            <FieldContent>
                                <Label label_for="name">"Name"</Label>
                                <FieldDescription>
                                    "Provide your full name for identification"
                                </FieldDescription>
                            </FieldContent>
                            <Input id="name" placeholder="Evil Rabbit" required=true />
                        </Field>
                        <Field orientation="responsive">
                            <FieldContent>
                                <Label>"Message"</Label>
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
