use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LinkButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-6">
            <Link as_button=true size="sm" class="text-(--muted-foreground)" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link as_button=true variant="primary" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link as_button=true variant="secondary" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link as_button=true variant="outline" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
        </div>
    }
}
