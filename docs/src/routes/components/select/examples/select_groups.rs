use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SelectGroupsExample() -> impl IntoView {
    view! {
        <Select>
            <SelectOption value="">"Select department"</SelectOption>
            <SelectOptGroup label="Engineering">
                <SelectOption value="frontend">"Frontend"</SelectOption>
                <SelectOption value="backend">"Backend"</SelectOption>
                <SelectOption value="devops">"DevOps"</SelectOption>
            </SelectOptGroup>
            <SelectOptGroup label="Sales">
                <SelectOption value="sales-rep">"Sales Rep"</SelectOption>
                <SelectOption value="account-manager">"Account Manager"</SelectOption>
                <SelectOption value="sales-director">"Sales Director"</SelectOption>
            </SelectOptGroup>
            <SelectOptGroup label="Operations">
                <SelectOption value="support">"Customer Support"</SelectOption>
                <SelectOption value="product-manager">"Product Manager"</SelectOption>
                <SelectOption value="ops-manager">"Operations Manager"</SelectOption>
            </SelectOptGroup>
        </Select>
    }
}
