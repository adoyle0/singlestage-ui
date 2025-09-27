use leptos::prelude::*;
use singlestage::{checkbox::*, Label};

#[component]
pub fn CheckboxExample() -> impl IntoView {
    view! {
        <Label>
            <Checkbox />
            "Accept terms and conditions"
        </Label>
    }
}
