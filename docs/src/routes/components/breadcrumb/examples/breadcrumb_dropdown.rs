use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbDropdownExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <Link href="/">Home</Link>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <DropdownMenu>
                        <Trigger>
                            <Button class="flex items-center gap-1" variant="none">
                                "Components"
                                {icon!(icondata::LuChevronDown, class="size-3.5")}
                            </Button>
                        </Trigger>
                        <MenuContent align="start">
                            <MenuGroup>
                                <MenuItem>"Documentation"</MenuItem>
                                <MenuItem>"Themes"</MenuItem>
                                <MenuItem>"GitHub"</MenuItem>
                            </MenuGroup>
                        </MenuContent>
                    </DropdownMenu>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
