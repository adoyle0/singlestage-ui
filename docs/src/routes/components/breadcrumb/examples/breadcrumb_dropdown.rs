use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BreadcrumbDropdownExample() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink>
                        <Link href="/">Home</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            <Button class="flex items-center gap-1" variant="none">
                                "Components"
                                {icon!(icondata::LuChevronDown, class="size-3.5")}
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
                <BreadcrumbSeparator>{icon!(icondata::LuDot)}</BreadcrumbSeparator>
                <BreadcrumbItem>
                    <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
