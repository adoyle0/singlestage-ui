use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupSizeExample() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start gap-8">
            <ButtonGroup>
                <Button variant="outline" size="sm">
                    "Small"
                </Button>
                <Button variant="outline" size="sm">
                    "Button"
                </Button>
                <Button variant="outline" size="sm">
                    "Group"
                </Button>
                <Button variant="outline" size="icon-sm">
                    {icon!(icondata::LuPlus)}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline">"Default"</Button>
                <Button variant="outline">"Button"</Button>
                <Button variant="outline">"Group"</Button>
                <Button variant="outline" size="icon">
                    {icon!(icondata::LuPlus)}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant="outline" size="lg">
                    "Large"
                </Button>
                <Button variant="outline" size="lg">
                    "Button"
                </Button>
                <Button variant="outline" size="lg">
                    "Group"
                </Button>
                <Button variant="outline" size="icon-lg">
                    {icon!(icondata::LuPlus)}
                </Button>
            </ButtonGroup>
        </div>
    }
}
