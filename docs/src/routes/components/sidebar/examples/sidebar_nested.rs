use leptos::prelude::*;
use singlestage::*;

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
                    <DropdownMenuContent align="end" side="right-top">
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

#[derive(Clone)]
struct NavItem {
    title: String,
    icon: icondata::Icon,
}

#[component]
fn OuterSidebar(
    active: RwSignal<String>,
    items: StoredValue<Vec<NavItem>>,
    user: StoredValue<User>,
) -> impl IntoView {
    view! {
        <Sidebar collapsible="icon" default_open=false>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <Link size="lg" href="url" class="md:h-8 md:p-0">
                                <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-(--sidebar-primary) text-(--sidebar-primary-foreground)">
                                    {icon!(icondata::LuCommand, class="size-4")}
                                </div>
                            </Link>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent class="px-1.5 md:px-0">
                        <SidebarMenu>
                            <For
                                each=move || items.get_value()
                                key=|item| item.title.clone()
                                children=move |item: NavItem| {
                                    let is_active = &item.title == &active.get();

                                    view! {
                                        <SidebarMenuItem>
                                            <SidebarMenuButton tooltip=item.title>
                                                <Button class=format!(
                                                    "{}",
                                                    if is_active {
                                                        "bg-(--sidebar-accent) text-(--sidebar-accent-foreground)"
                                                    } else {
                                                        ""
                                                    },
                                                )>
                                                    {
                                                        let icon = item.icon;
                                                        icon!(icon)
                                                    }
                                                </Button>
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    }
                                }
                            />
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter>
                <NavUser user />
            </SidebarFooter>
        </Sidebar>
    }
}

#[derive(Clone)]
struct Mail {
    name: String,
    email: String,
    subject: String,
    date: String,
    teaser: String,
}

#[component]
fn InnerSidebar(active: RwSignal<String>, mails: StoredValue<Vec<Mail>>) -> impl IntoView {
    view! {
        <Sidebar class="rounded-none!">
            <SidebarHeader class="gap-3.5 border-b p-4">
                <div class="flex w-full items-center justify-between">
                    <div class="text-base font-medium text-foreground">{move || active.get()}</div>
                    <Label class="flex items-center gap-2 text-sm">
                        <span>"Unreads"</span>
                        <Switch class="shadow-none" />
                    </Label>
                </div>
                <Input placeholder="Type to search..." />
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup class="px-0">
                    <SidebarGroupContent>
                        <For
                            each=move || mails.get_value()
                            key=|mail| mail.subject.clone()
                            let(mail)
                        >
                            <a class="flex flex-col items-start gap-2 border-b p-4 text-sm leading-tight whitespace-nowrap last:border-b-0 hover:bg-(--sidebar-accent) hover:text-(--sidebar-accent-foreground)">
                                <div class="flex w-full items-center gap-2">
                                    <span class="">{mail.name}</span>
                                    " "
                                    <span class="ml-auto text-xs">{mail.date}</span>
                                </div>
                                <span class="font-medium">{mail.subject}</span>
                                <span class="line-clamp-2 w-[260px] text-xs whitespace-break-spaces">
                                    {mail.teaser}
                                </span>
                            </a>
                        </For>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    }
}

#[component]
pub fn SidebarNestedExample() -> impl IntoView {
    let user = StoredValue::new(User {
        name: "Sofia Davis".to_string(),
        email: "m@example.com".to_string(),
        avatar: "/avatar-1.png".to_string(),
    });

    let items = StoredValue::new(vec![
        NavItem {
            title: "Inbox".to_string(),
            icon: icondata::LuInbox,
        },
        NavItem {
            title: "Drafts".to_string(),
            icon: icondata::LuFile,
        },
        NavItem {
            title: "Sent".to_string(),
            icon: icondata::LuSend,
        },
        NavItem {
            title: "Junk".to_string(),
            icon: icondata::LuArchiveX,
        },
        NavItem {
            title: "Trash".to_string(),
            icon: icondata::LuTrash2,
        },
    ]);

    let mails = StoredValue::new(vec![
        Mail {
            name: "William Smith".to_string(),
            email: "williamsmith@example.com".to_string(),
            subject: "Meeting Tomorrow".to_string(),
            date: "09:34 AM".to_string(),
            teaser: "Hi team, just a reminder about our meeting tomorrow at 10 AM.\nPlease come prepared with your project updates.".to_string(),
        },
        Mail {
            name: "Alice Smith".to_string(),
            email: "alicesmith@example.com".to_string(),
            subject: "Re: Project Update".to_string(),
            date: "Yesterday".to_string(),
            teaser: "Thanks for the update. The progress looks great so far.\nLet's schedule a call to discuss the next steps.".to_string(),
        },
        Mail {
            name: "Bob Johnson".to_string(),
            email: "bobjohnson@example.com".to_string(),
            subject: "Weekend Plans".to_string(),
            date: "2 days ago".to_string(),
            teaser: "Hey everyone! I'm thinking of organizing a team outing this weekend.\nWould you be interested in a hiking trip or a beach day?".to_string(),
        },
        Mail {
            name: "Emily Davis".to_string(),
            email: "emilydavis@example.com".to_string(),
            subject: "Re: Question about Budget".to_string(),
            date: "2 days ago".to_string(),
            teaser: "I've reviewed the budget numbers you sent over.\nCan we set up a quick call to discuss some potential adjustments?".to_string(),
        },
        Mail {
            name: "Michael Wilson".to_string(),
            email: "michaelwilson@example.com".to_string(),
            subject: "Important Announcement".to_string(),
            date: "1 week ago".to_string(),
            teaser: "Please join us for an all-hands meeting this Friday at 3 PM.\nWe have some exciting news to share about the company's future.".to_string(),
        },
        Mail {
            name: "Sarah Brown".to_string(),
            email: "sarahbrown@example.com".to_string(),
            subject: "Re: Feedback on Proposal".to_string(),
            date: "1 week ago".to_string(),
            teaser: "Thank you for sending over the proposal. I've reviewed it and have some thoughts.\nCould we schedule a meeting to discuss my feedback in detail?".to_string(),
        },
        Mail {
            name: "David Lee".to_string(),
            email: "davidlee@example.com".to_string(),
            subject: "New Project Idea".to_string(),
            date: "1 week ago".to_string(),
            teaser: "I've been brainstorming and came up with an interesting project concept.\nDo you have time this week to discuss its potential impact and feasibility?".to_string(),
        },
        Mail {
            name: "Olivia Wilson".to_string(),
            email: "oliviawilson@example.com".to_string(),
            subject: "Vacation Plans".to_string(),
            date: "1 week ago".to_string(),
            teaser: "Just a heads up that I'll be taking a two-week vacation next month.\nI'll make sure all my projects are up to date before I leave.".to_string(),
        },
        Mail {
            name: "James Martin".to_string(),
            email: "jamesmartin@example.com".to_string(),
            subject: "Re: Conference Registration".to_string(),
            date: "1 week ago".to_string(),
            teaser: "I've completed the registration for the upcoming tech conference.\nLet me know if you need any additional information from my end.".to_string(),
        },
        Mail {
            name: "Sophia White".to_string(),
            email: "sophiawhite@example.com".to_string(),
            subject: "Team Dinner".to_string(),
            date: "1 week ago".to_string(),
            teaser: "To celebrate our recent project success, I'd like to organize a team dinner.\nAre you available next Friday evening? Please let me know your preferences.".to_string(),
        },
    ]);

    let is_mobile = RwSignal::new(false);
    let active = RwSignal::new(items.get_value()[0].title.clone());

    view! {
        <SidebarProvider is_mobile>
            <OuterSidebar active items user />
            <SidebarProvider style="--sidebar-width: 300px">
                <InnerSidebar active mails />
                <SidebarInset>
                    <header class="sticky top-0 flex shrink-0 items-center gap-2 border-b bg-background p-2">
                        <SidebarTrigger />
                        <Separator orientation="vertical" class="mr-2 h-4" />
                        <Breadcrumb>
                            <BreadcrumbList>
                                <Show when=move || !is_mobile.get()>
                                    <BreadcrumbItem>
                                        <BreadcrumbLink>
                                            <Link>"All Inboxes"</Link>
                                        </BreadcrumbLink>
                                    </BreadcrumbItem>
                                    <BreadcrumbSeparator />
                                </Show>
                                <BreadcrumbItem>
                                    <BreadcrumbPage>"Inbox"</BreadcrumbPage>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>
                    </header>
                    <div class="flex flex-1 flex-col gap-4 p-4">
                        <Skeleton class="h-24" />
                        <Skeleton class="flex-1 md:min-h-min" />
                    </div>
                // <div class="flex flex-1 flex-col gap-4 p-4 overflow-y-scroll">
                // <For each=move || 0..24 key=|k| k.clone() let(_)>
                // <Skeleton class="min-h-12 w-full rounded-lg" />
                // </For>
                // </div>
                </SidebarInset>
            </SidebarProvider>
        </SidebarProvider>
    }
}
