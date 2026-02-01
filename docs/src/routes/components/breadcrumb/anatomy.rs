use leptos::prelude::*;
use singlestage*;

#[component]
pub fn BreadcrumbAnatomy() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem />
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbEllipsis />
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    {icon!(icondata::LuDot)}
                </BreadcrumbSeparator>
                <BreadcrumbPage />
            </BreadcrumbList>
        </Breadcrumb>
    }
}
