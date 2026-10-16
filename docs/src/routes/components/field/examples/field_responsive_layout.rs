use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldResponsiveLayoutExample() -> impl IntoView {
    view! {
        <FieldSet class="max-w-lg">
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
                    <Button>"Submit"</Button>
                    <Button variant="outline">"Cancel"</Button>
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
