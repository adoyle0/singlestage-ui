use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LinkButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-6">
            <Link render_as="button" size="sm" class="text-(--muted-foreground)" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link render_as="button" variant="primary" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link render_as="button" variant="secondary" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
            <Link render_as="button" variant="outline" href="#">
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
        </div>
    }
}
