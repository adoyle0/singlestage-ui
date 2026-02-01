use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbCustomSeparatorExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <Link href="/">"Home"</Link>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <Link href="/components">"Components"</Link>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
