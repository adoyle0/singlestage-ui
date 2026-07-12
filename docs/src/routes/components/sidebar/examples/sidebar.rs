use leptos::prelude::*;
use singlestage::*;

#[derive(Clone)]
struct Team {
    name: String,
    logo: icondata::Icon,
    plan: String,
}

#[component]
fn TeamSwitcher(teams: StoredValue<Vec<Team>>) -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    let active_team = RwSignal::new(teams.get_value()[0].clone());

    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <SidebarMenuButton>
                            <Button
                                size="lg"
                                class="aria-expanded:bg-(--sidebar-accent) aria-expanded:text-(--sidebar-accent-foreground)"
                            >
                                <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-(--sidebar-primary) text-(--sidebar-primary-foreground)">
                                    {move || {
                                        let logo = active_team.get().logo;
                                        icon!(logo, class="size-4")
                                    }}
                                </div>
                                <div class="grid flex-1 text-left text-sm leading-tight">
                                    <span class="truncate font-medium">
                                        {move || active_team.get().name}
                                    </span>
                                    <span class="truncate text-xs">
                                        {move || active_team.get().plan}
                                    </span>
                                </div>
                                {icon!(icondata::LuChevronsUpDown, class="ml-auto")}
                            </Button>
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        class="w-(--sidebar-width) min-w-56 rounded-lg"
                        side="bottom"
                    >
                        <DropdownMenuLabel class="text-xs text-muted-foreground">
                            "Teams"
                        </DropdownMenuLabel>
                        <For each=move || teams.get_value() key=|team| team.name.clone() let(team)>
                            {
                                let team_clone = team.clone();
                                view! {
                                    <DropdownMenuItem
                                        class="gap-2 p-2"
                                        on:click=move |_| active_team.set(team_clone.to_owned())
                                    >

                                        <div class="flex size-6 items-center justify-center rounded-md border">
                                            {
                                                let logo = team.logo;
                                                icon!(logo, class="size-3.5 shrink-0")
                                            }
                                        </div>
                                        {team.name}
                                    </DropdownMenuItem>
                                }
                            }
                        </For>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem class="gap-2 p-2">
                            <div class="flex size-6 items-center justify-center rounded-md border bg-transparent">
                                {icon!(icondata::LuPlus, class="size-4")}
                            </div>
                            <div class="font-medium text-muted-foreground">"Add team"</div>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}

#[derive(Clone)]
struct SubItem {
    title: String,
    url: String,
}

#[derive(Clone)]
struct Item {
    title: String,
    icon: icondata::Icon,
    is_active: bool,
    items: Vec<SubItem>,
}

#[component]
fn NavMain(items: StoredValue<Vec<Item>>) -> impl IntoView {
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"Platform"</SidebarGroupLabel>
            <SidebarMenu>
                <For each=move || items.get_value() key=|item| item.title.clone() let(item)>
                    <Collapsible open={item.is_active}>
                        <SidebarMenuItem>
                            <CollapsibleTrigger>
                                <SidebarMenuButton>
                                    <Button class="aria-expanded:[&_svg]:rotate-90">
                                        {
                                            let icon = item.icon;
                                            icon!(icon)
                                        } <span>{item.title}</span>
                                        {icon!(
                                            icondata::LuChevronRight, class="ml-auto transition-transform duration-200"
                                        )}
                                    </Button>
                                </SidebarMenuButton>
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenuSub>
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
                            </CollapsibleContent>
                        </SidebarMenuItem>
                    </Collapsible>
                </For>
            </SidebarMenu>
        </SidebarGroup>
    }
}

#[derive(Clone)]
struct Project {
    name: String,
    icon: icondata::Icon,
    url: String,
}

#[component]
fn NavProjects(projects: StoredValue<Vec<Project>>) -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Show when=move || sidebar.open.get()>
            <SidebarGroup class="group-data-[collapsible=icon]:hidden">
                <SidebarGroupLabel>"Projects"</SidebarGroupLabel>
                <SidebarMenu>
                    <For
                        each=move || projects.get_value()
                        key=|project| project.name.clone()
                        let(project)
                    >
                        <SidebarMenuItem>
                            <SidebarMenuButton>
                                <Link href={project
                                    .url}>
                                    {
                                        let icon = project.icon;
                                        icon!(icon)
                                    } <span>{project.name}</span>
                                </Link>
                            </SidebarMenuButton>
                            <DropdownMenu>
                                <DropdownMenuTrigger>
                                    <SidebarMenuAction show_on_hover=true>
                                        // <Button size="sm" variant="ghost">
                                        {icon!(icondata::FiMoreHorizontal)}
                                        <span class="sr-only">"More"</span>
                                    // </Button>
                                    </SidebarMenuAction>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent
                                    class="w-48 rounded-lg"
                                    align=match sidebar.is_mobile.get_untracked() {
                                        true => "end",
                                        false => "start",
                                    }
                                    side=match sidebar.is_mobile.get_untracked() {
                                        true => "bottom",
                                        false => "right",
                                    }
                                >
                                    <DropdownMenuItem>
                                        {icon!(icondata::LuFolder, class="text-muted-foreground")}
                                        <span>"View Project"</span>
                                    </DropdownMenuItem>
                                    <DropdownMenuItem>
                                        {icon!(icondata::LuForward, class="text-muted-foreground")}
                                        <span>"Share Project"</span>
                                    </DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem>
                                        {icon!(icondata::LuTrash2, class="text-(muted-foreground)")}
                                        <span>"Delete Project"</span>
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </SidebarMenuItem>
                    </For>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <Button class="text-sidebar-foreground/70">
                                {icon!(icondata::FiMoreHorizontal)} <span>"More"</span>
                            </Button>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroup>
        </Show>
    }
}

#[derive(Clone)]
struct User {
    name: String,
    email: String,
    avatar: String,
}

#[component]
fn NavUser(user: StoredValue<User>) -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

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
                                <Avatar class="h-8 w-8 rounded-lg">
                                    <AvatarImage
                                        src={user.get_value().avatar}
                                        alt={user.get_value().name}
                                    />
                                    <AvatarFallback class="rounded-lg">"SD"</AvatarFallback>
                                </Avatar>
                                <div class="grid flex-1 text-left text-sm leading-tight">
                                    <span class="truncate font-medium">
                                        {user.get_value().name}
                                    </span>
                                    <span class="truncate text-xs">{user.get_value().email}</span>
                                </div>
                                {icon!(icondata::LuChevronsUpDown, class="ml-auto size-4")}
                            </Button>
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        class="min-w-56 rounded-lg"
                        align="end"
                        side=match sidebar.is_mobile.get_untracked() {
                            true => "top",
                            false => "right-top",
                        }
                    >
                        <DropdownMenuLabel class="p-0 font-normal">
                            <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                                <Avatar class="h-8 w-8 rounded-lg">
                                    <AvatarImage
                                        src={user.get_value().avatar}
                                        alt={user.get_value().name}
                                    />
                                    <AvatarFallback class="rounded-lg">"CN"</AvatarFallback>
                                </Avatar>
                                <div class="grid flex-1 text-left text-sm leading-tight">
                                    <span class="truncate font-medium">
                                        {user.get_value().name}
                                    </span>
                                    <span class="truncate text-xs">{user.get_value().email}</span>
                                </div>
                            </div>
                        </DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem>
                                {icon!(icondata::LuSparkles)} "Upgrade to Pro"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem>
                                {icon!(icondata::LuBadgeCheck)} "Account"
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                {icon!(icondata::LuCreditCard)} "Billing"
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                {icon!(icondata::LuBell)} "Notifications"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>{icon!(icondata::LuLogOut)} "Log out"</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}

#[component]
fn AppSidebar() -> impl IntoView {
    let user = StoredValue::new(User {
        name: "Sofia Davis".to_string(),
        email: "m@example.com".to_string(),
        avatar: "/avatar-1.png".to_string(),
    });

    let teams = StoredValue::new(vec![
        Team {
            name: "Acme Inc".to_string(),
            logo: icondata::LuGalleryVerticalEnd,
            plan: "Enterprise".to_string(),
        },
        Team {
            name: "Acme Corp.".to_string(),
            logo: icondata::LuAudioWaveform,
            plan: "Startup".to_string(),
        },
        Team {
            name: "Evil Corp.".to_string(),
            logo: icondata::LuCommand,
            plan: "Free".to_string(),
        },
    ]);

    let items = StoredValue::new(vec![
        Item {
            title: "Playground".to_string(),
            icon: icondata::LuSquareTerminal,
            is_active: true,
            items: vec![
                SubItem {
                    title: "History".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Starred".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Settings".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        Item {
            title: "Models".to_string(),
            icon: icondata::LuBot,
            is_active: false,
            items: vec![
                SubItem {
                    title: "Genesis".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Explorer".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Quantum".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        Item {
            title: "Documentation".to_string(),
            icon: icondata::LuBookOpen,
            is_active: false,
            items: vec![
                SubItem {
                    title: "Introduction".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Get Started".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Tutorials".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Changelog".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        Item {
            title: "Settings".to_string(),
            icon: icondata::LuSettings2,
            is_active: false,
            items: vec![
                SubItem {
                    title: "General".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Team".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Billing".to_string(),
                    url: "url".to_string(),
                },
                SubItem {
                    title: "Limits".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
    ]);

    let projects = StoredValue::new(vec![
        Project {
            name: "Design Engineering".to_string(),
            url: "url".to_string(),
            icon: icondata::LuFrame,
        },
        Project {
            name: "Sales & Marketing".to_string(),
            url: "url".to_string(),
            icon: icondata::FiPieChart,
        },
        Project {
            name: "Travel".to_string(),
            url: "url".to_string(),
            icon: icondata::LuMap,
        },
    ]);

    view! {
        <Sidebar collapsible="icon">
            <SidebarHeader>
                <TeamSwitcher teams />
            </SidebarHeader>
            <SidebarContent>
                <NavMain items />
                <NavProjects projects />
            </SidebarContent>
            <SidebarFooter>
                <NavUser user />
            </SidebarFooter>
            <SidebarRail />
        </Sidebar>
    }
}

#[component]
pub fn SidebarExample() -> impl IntoView {
    view! {
        <SidebarProvider>
            <AppSidebar />
            <SidebarInset>
                <header class="flex h-16 shrink-0 items-center gap-2 border-b px-2">
                    <SidebarTrigger />
                    <Separator orientation="vertical" class="mr-2 h-4" />
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink>
                                    <Link href="#">"Build Your Application"</Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
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
                    <Skeleton class="min-h-[100vh] flex-1 md:min-h-min" />
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}
