use leptos::prelude::*;
use singlestage::button::*;

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
