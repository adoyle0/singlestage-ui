use leptos::prelude::*;
use singlestage::*;

#[derive(Clone, PartialEq)]
struct Team {
    name: String,
    logo: icondata::Icon,
    plan: String,
}

#[component]
fn TeamSwitcher(teams: StoredValue<Vec<Team>>) -> impl IntoView {
    let active_team = RwSignal::new(0);

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
                                {move || {
                                    let active_team = &teams.get_value()[active_team.get()];

                                    view! {
                                        <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-(--sidebar-primary) text-(--sidebar-primary-foreground)">
                                            {
                                                let logo = active_team.logo;
                                                icon!(logo, class="size-4")
                                            }
                                        </div>
                                        <div class="grid flex-1 text-left text-sm leading-tight">
                                            <span class="truncate font-medium">
                                                {active_team.name.to_owned()}
                                            </span>
                                            <span class="truncate text-xs">
                                                {active_team.plan.to_owned()}
                                            </span>
                                        </div>
                                    }
                                }}
                                {icon!(icondata::LuChevronsUpDown, class="ml-auto")}
                            </Button>
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent class="min-w-58 rounded-lg" side="bottom" align="center">
                        <DropdownMenuLabel class="text-xs text-muted-foreground">
                            "Teams"
                        </DropdownMenuLabel>
                        <For
                            each=move || teams.get_value()
                            key=|team| team.name.clone()
                            children=move |team: Team| {
                                let index = teams
                                    .get_value()
                                    .iter()
                                    .position(|t| t == &team)
                                    .unwrap_or_default();

                                view! {
                                    <DropdownMenuItem
                                        class="gap-2 p-2"
                                        on:click={move |_| { active_team.set(index) }}
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
                        />
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
struct NavSubItem {
    title: String,
    url: String,
}

#[derive(Clone)]
struct NavItem {
    title: String,
    icon: icondata::Icon,
    is_active: bool,
    items: Vec<NavSubItem>,
}

#[component]
fn NavMain(items: StoredValue<Vec<NavItem>>) -> impl IntoView {
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"Platform"</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    <For
                        each=move || items.get_value()
                        key=|item| item.title.clone()
                        children=|item: NavItem| {
                            let tooltip = item.title.clone();
                            view! {
                                <Collapsible open={item.is_active}>
                                    <SidebarMenuItem>
                                        <CollapsibleTrigger>
                                            <SidebarMenuButton tooltip>
                                                <Button class="aria-expanded:[&>svg:last-child]:rotate-90">
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
                            }
                        }
                    />
                </SidebarMenu>
            </SidebarGroupContent>
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
                                        {icon!(icondata::FiMoreHorizontal)}
                                        <span class="sr-only">"More"</span>
                                    </SidebarMenuAction>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent
                                    class="min-w-58 rounded-lg"
                                    align="end"
                                    side="bottom"
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
struct NavSecondary {
    title: String,
    url: String,
    icon: icondata::Icon,
}

#[component]
fn NavSecondary(items: StoredValue<Vec<NavSecondary>>) -> impl IntoView {
    view! {
        <SidebarGroup class="mt-auto">
            <SidebarGroupContent>
                <SidebarMenu>
                    <For each=move || items.get_value() key=|item| item.title.clone() let(item)>
                        <SidebarMenuItem>
                            <SidebarMenuButton>
                                <Link size="sm" href=item.url>
                                    {
                                        let icon = item.icon;
                                        icon!(icon)
                                    }
                                    <span>{item.title}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </For>
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
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
                    <DropdownMenuContent class="min-w-58 rounded-lg" align="center" side="top">
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
        NavItem {
            title: "Playground".to_string(),
            icon: icondata::LuSquareTerminal,
            is_active: true,
            items: vec![
                NavSubItem {
                    title: "History".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Starred".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Settings".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Models".to_string(),
            icon: icondata::LuBot,
            is_active: false,
            items: vec![
                NavSubItem {
                    title: "Genesis".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Explorer".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Quantum".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Documentation".to_string(),
            icon: icondata::LuBookOpen,
            is_active: false,
            items: vec![
                NavSubItem {
                    title: "Introduction".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Get Started".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Tutorials".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Changelog".to_string(),
                    url: "url".to_string(),
                },
            ],
        },
        NavItem {
            title: "Settings".to_string(),
            icon: icondata::LuSettings2,
            is_active: false,
            items: vec![
                NavSubItem {
                    title: "General".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Team".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
                    title: "Billing".to_string(),
                    url: "url".to_string(),
                },
                NavSubItem {
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

    let secondary = StoredValue::new(vec![
        NavSecondary {
            title: "Support".to_string(),
            url: "url".to_string(),
            icon: icondata::LuLifeBuoy,
        },
        NavSecondary {
            title: "Feedback".to_string(),
            url: "url".to_string(),
            icon: icondata::LuSend,
        },
    ]);

    view! {
        <Sidebar variant="inset">
            <SidebarHeader>
                <TeamSwitcher teams />
            </SidebarHeader>
            <SidebarContent>
                <NavMain items />
                <NavProjects projects />
                <NavSecondary items=secondary />
            </SidebarContent>
            <SidebarFooter>
                <NavUser user />
            </SidebarFooter>
            <SidebarRail />
        </Sidebar>
    }
}

#[component]
pub fn SidebarSecondaryExample() -> impl IntoView {
    let is_mobile = RwSignal::new(false);

    view! {
        <SidebarProvider is_mobile>
            <AppSidebar />
            <SidebarInset>
                <header class="flex h-16 shrink-0 items-center gap-2 border-b px-2">
                    <SidebarTrigger />
                    <Separator orientation="vertical" class="mr-2 h-4" />
                    <Breadcrumb>
                        <BreadcrumbList>
                            <Show when=move || !is_mobile.get()>
                                <BreadcrumbItem>
                                    <BreadcrumbLink>
                                        <Link href="#">"Build Your Application"</Link>
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
