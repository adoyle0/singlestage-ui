use leptos::prelude::*;
use singlestage::breadcrumb::*;

#[component]
pub fn BreadcrumbAnatomy() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem />
            <BreadcrumbSeparator />
        </Breadcrumb>
    }
}
