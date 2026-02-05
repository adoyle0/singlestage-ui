use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <Link href="#">"Home"</Link>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <DropdownMenu>
                        <Trigger>
                            <Button size="icon-sm" variant="ghost">
                                <BreadcrumbEllipsis />
                                <span class="sr-only">"Toggle menu"</span>
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
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <Link href="#">"Components"</Link>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
