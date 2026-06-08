use leptos::prelude::*;
use singlestage::select::*;

#[component]
pub fn SelectAnatomy() -> impl IntoView {
    view! {
        <Select>
            <SelectOptGroup>
                <SelectOption />
            </SelectOptGroup>
        </Select>
    }
}
