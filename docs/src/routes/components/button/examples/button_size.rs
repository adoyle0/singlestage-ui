use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonSizeExample() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start gap-8 sm:flex-row">
            <div class="flex items-start gap-2">
                <Button size="xs" variant="outline">
                    "Extra Small"
                </Button>
                <Button size="icon-xs" aria_label="Submit" variant="outline">
                    {icon!(icondata::LuArrowUpRight)}
                </Button>
            </div>
            <div class="flex items-start gap-2">
                <Button size="sm" variant="outline">
                    "Small"
                </Button>
                <Button size="icon-sm" aria_label="Submit" variant="outline">
                    {icon!(icondata::LuArrowUpRight)}
                </Button>
            </div>
            <div class="flex items-start gap-2">
                <Button variant="outline">Default</Button>
                <Button size="icon" aria_label="Submit" variant="outline">
                    {icon!(icondata::LuArrowUpRight)}
                </Button>
            </div>
            <div class="flex items-start gap-2">
                <Button variant="outline" size="lg">
                    "Large"
                </Button>
                <Button size="icon-lg" aria_label="Submit" variant="outline">
                    {icon!(icondata::LuArrowUpRight)}
                </Button>
            </div>
        </div>
    }
}
