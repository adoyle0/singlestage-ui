use leptos::prelude::*;
use singlestage::{Button, breadcrumb::*, dropdown::*, icon};

#[component]
pub fn BreadcrumbExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <a href="#" class="hover:text-(--foreground) transition-colors">
                    "Home"
                </a>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <Button size="icon-sm" variant="ghost">
                            {icon!(icondata::VsEllipsis)}
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
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
                <a href="#" class="hover:text-(--foreground) transition-colors">
                    "Components"
                </a>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <span class="text-(--foreground) font-normal">"Breadcrumb"</span>
            </BreadcrumbItem>
        </Breadcrumb>
    }
}
