use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupSplitExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="secondary">"Button"</Button>
            <Separator />
            <Button size="icon" variant="secondary">
                {icon!(icondata::LuPlus)}
            </Button>
        </ButtonGroup>
    }
}
