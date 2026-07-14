use leptos::prelude::*;
use singlestage::*;

#[component]
fn VersionSwitcher(versions: StoredValue<Vec<String>>) -> impl IntoView {
    let selected_version = RwSignal::new(0);

    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <SidebarMenuButton>
                            <Button
                                size="lg"
                                class="aria-expanded:bg-sidebar-accent aria-expanded:text-sidebar-accent-foreground"
                            >
                                <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                                    {icon!(icondata::LuGalleryVerticalEnd, class="size-4")}
                                </div>
                                <div class="flex flex-col gap-0.5 leading-none">
                                    <span class="font-medium">"Documentation"</span>
                                    <span>
                                        "v"
                                        {move || {
                                            versions.get_value()[selected_version.get()].to_owned()
                                        }}
                                    </span>
                                </div>
                                {icon!(icondata::LuChevronsUpDown, class="ml-auto")}
                            </Button>
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent class="min-w-58" align="center" side="bottom">
                        <For
                            each=move || versions.get_value()
                            key=|version| version.clone()
                            children=move |version: String| {
                                let index = versions
                                    .get_value()
                                    .iter()
                                    .position(|v| v == &version)
                                    .unwrap_or_default();

                                view! {
                                    <DropdownMenuItem on:click=move |_| {
                                        selected_version.set(index)
                                    }>
                                        "v"{version.clone()}" "
                                        <Show when=move || {
                                            version == versions.get_value()[selected_version.get()]
                                        }>{icon!(icondata::LuCheck, class="ml-auto")}</Show>
                                    </DropdownMenuItem>
                                }
                            }
                        />
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}

#[derive(Clone)]
struct NavSubItem {
    title: String,
    url: String,
}

#[derive(Clone)]
struct NavItem {
    title: String,
    items: Vec<NavSubItem>,
}

#[component]
fn AppSidebar() -> impl IntoView {
    let versions = StoredValue::new(vec![
        "1.0.1".to_string(),
        "1.1.0-alpha".to_string(),
        "2.0.0-beta1".to_string(),
    ]);

    let nav_main = StoredValue::new(vec![
        NavItem {
            title: "Getting Started".to_string(),
            items: vec![
                NavSubItem {
                    title: "Installation".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Project Structure".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Build Your Application".to_string(),
            items: vec![
                NavSubItem {
                    title: "Routing".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Data Fetching".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Rendering".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Caching".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Styling".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Optimizing".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Configuring".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Testing".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Authentication".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Deploying".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Upgrading".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Examples".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "API Reference".to_string(),
            items: vec![
                NavSubItem {
                    title: "Components".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "File Conventions".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Functions".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Leptos Options".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "CLI".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Edge Runtime".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Architecture".to_string(),
            items: vec![
                NavSubItem {
                    title: "Accessibility".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Fast Refresh".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Cargo Leptos".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Supported Browsers".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Wasm Bindgen".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Community".to_string(),
            items: vec![NavSubItem {
                title: "Contribution Guide".to_string(),
                url: "url".to_string(),
            }],
        },
    ]);

    view! {
        <Sidebar variant="floating">
            <SidebarHeader>
                <VersionSwitcher versions />
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarMenu class="gap-2">
                        <For
                            each=move || nav_main.get_value()
                            key=|item| item.title.clone()
                            let(item)
                        >
                            <SidebarMenuItem>
                                <SidebarMenuButton>
                                    <Button class="font-medium">{item.title}</Button>
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                            <SidebarMenuSub class="ml-0 border-l-0 px-1.5">
                                <For
                                    each=move || item.items.to_owned()
                                    key=|item| item.title.clone()
                                    let(sub_item)
                                >
                                    <SidebarMenuSubItem>
                                        <SidebarMenuSubButton>
                                            <Link href={sub_item.url}>
                                                <span>{sub_item.title}</span>
                                            </Link>
                                        </SidebarMenuSubButton>
                                    </SidebarMenuSubItem>
                                </For>
                            </SidebarMenuSub>
                        </For>
                    </SidebarMenu>
                </SidebarGroup>
            </SidebarContent>
            <SidebarRail />
        </Sidebar>
    }
}

#[component]
pub fn SidebarFloatingSubmenusExample() -> impl IntoView {
    let is_mobile = RwSignal::new(false);

    view! {
        <SidebarProvider is_mobile>
            <AppSidebar />
            <SidebarInset>
                <header class="flex h-16 shrink-0 items-center gap-2 px-2">
                    <SidebarTrigger />
                    <Separator orientation="vertical" class="mr-2 h-4" />
                    <Breadcrumb>
                        <BreadcrumbList>
                            <Show when=move || !is_mobile.get()>
                                <BreadcrumbItem>
                                    <BreadcrumbLink>
                                        <Link>"Build Your Application"</Link>
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                            </Show>
                            <BreadcrumbItem>
                                <BreadcrumbPage>"Data Fetching"</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                </header>
                <div class="flex flex-1 flex-col gap-4 p-4">
                    <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                        <Skeleton class="aspect-video" />
                        <Skeleton class="aspect-video" />
                        <Skeleton class="aspect-video" />
                    </div>
                    <Skeleton class="flex-1 md:min-h-min" />
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}
