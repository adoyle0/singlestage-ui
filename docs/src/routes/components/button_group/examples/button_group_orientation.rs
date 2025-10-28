use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupOrientationExample() -> impl IntoView {
    view! {
        <ButtonGroup orientation="vertical" attr:aria-label="Media controls" class="h-fit">
            <Button variant="outline" size="icon">
                {icon!(icondata::LuPlus)}
            </Button>
            <Button variant="outline" size="icon">
                {icon!(icondata::LuMinus)}
            </Button>
        </ButtonGroup>
    }
}
