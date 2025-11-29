use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupNestedExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <ButtonGroup>
                <Button variant="outline" size="sm">
                    "1"
                </Button>
                <Button variant="outline" size="sm">
                    "2"
                </Button>
                <Button variant="outline" size="sm">
                    "3"
                </Button>
                <Button variant="outline" size="sm">
                    "4"
                </Button>
                <Button variant="outline" size="sm">
                    "5"
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline" size="icon-sm" aria_label="Previous">
                    {icon!(icondata::LuArrowLeft)}
                </Button>
                <Button variant="outline" size="icon-sm" aria_label="Next">
                    {icon!(icondata::LuArrowRight)}
                </Button>
            </ButtonGroup>
        </ButtonGroup>
    }
}
