use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink>
                        <Link href="#">"Home"</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            <Button size="icon-sm" variant="ghost">
                                <BreadcrumbEllipsis />
                                <span class="sr-only">"Toggle menu"</span>
                            </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="start">
                            <DropdownMenuGroup>
                                <DropdownMenuItem>"Documentation"</DropdownMenuItem>
                                <DropdownMenuItem>"Themes"</DropdownMenuItem>
                                <DropdownMenuItem>"GitHub"</DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink>
                        <Link href="#">"Components"</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
