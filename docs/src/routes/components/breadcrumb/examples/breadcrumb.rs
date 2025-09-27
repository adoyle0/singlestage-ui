use leptos::prelude::*;
use singlestage::{breadcrumb::*, dropdown::*, icon};

#[component]
pub fn BreadcrumbExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <a href="#" class="hover:text-foreground transition-colors">
                    "Home"
                </a>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <DropdownMenu>
                    <DropdownMenuTrigger
                        class="flex size-9 items-center justify-center h-4 w-4 hover:text-foreground cursor-pointer"
                        variant="ghost"
                    >
                        {icon!(icondata::VsEllipsis)}
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
                <a href="#" class="hover:text-foreground transition-colors">
                    "Components"
                </a>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <span class="text-foreground font-normal">"Breadcrumb"</span>
            </BreadcrumbItem>
        </Breadcrumb>
    }
}
