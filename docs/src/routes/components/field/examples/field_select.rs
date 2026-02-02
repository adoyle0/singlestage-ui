use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSelectExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldSet>
                <FieldGroup>
                    <Field>
                        <FieldLabel>"Department"</FieldLabel>
                        <Select placeholder="Choose department">
                            <SelectOptGroup>
                                <SelectOption value="engineering">"Engineering"</SelectOption>
                                <SelectOption value="design">"Design"</SelectOption>
                                <SelectOption value="marketing">"Marketing"</SelectOption>
                                <SelectOption value="sales">"Sales"</SelectOption>
                                <SelectOption value="support">"Customer Support"</SelectOption>
                                <SelectOption value="hr">"Human Resources"</SelectOption>
                                <SelectOption value="finance">"Finance"</SelectOption>
                                <SelectOption value="operations">"Operations"</SelectOption>
                            </SelectOptGroup>
                        </Select>
                        <FieldDescription>
                            "Select your department or area of work."
                        </FieldDescription>
                    </Field>
                </FieldGroup>
            </FieldSet>
        </div>
    }
}
