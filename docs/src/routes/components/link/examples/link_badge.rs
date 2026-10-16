use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LinkBadgeExample() -> impl IntoView {
    view! {
        <Link render_as="badge" href="#link">
            <span>"Open Link "</span>
            {icon!(icondata::LuArrowUpRight)}
        </Link>
    }
}
