use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbCustomSeparatorExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink>
                        <Link href="/">"Home"</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbLink>
                        <Link href="/components">"Components"</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
