use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldSelectExample() -> impl IntoView {
    view! {
        <Field class="max-w-xs">
            <FieldLabel>"Department"</FieldLabel>
            <Select placeholder="Choose department">
                <SelectOption value="engineering">"Engineering"</SelectOption>
                <SelectOption value="design">"Design"</SelectOption>
                <SelectOption value="marketing">"Marketing"</SelectOption>
                <SelectOption value="sales">"Sales"</SelectOption>
                <SelectOption value="support">"Customer Support"</SelectOption>
                <SelectOption value="hr">"Human Resources"</SelectOption>
                <SelectOption value="finance">"Finance"</SelectOption>
                <SelectOption value="operations">"Operations"</SelectOption>
            </Select>
            <FieldDescription>"Select your department or area of work."</FieldDescription>
        </Field>
    }
}
