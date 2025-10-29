use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupAnatomy() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button />
            <ButtonGroupSeparator />
            <ButtonGroupText />
        </ButtonGroup>
    }
}
