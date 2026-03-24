use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupSeparatorExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="secondary" size="sm">
                "Copy"
            </Button>
            <ButtonGroupSeparator />
            <Button variant="secondary" size="sm">
                "Paste"
            </Button>
        </ButtonGroup>
    }
}
