use leptos::prelude::*;
use singlestage::*;

/// Converts string To Title Case
fn string_to_title(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    let mut input_iter = input.char_indices();

    while let Some((index, mut char)) = input_iter.next() {
        if index == 0 {
            char.make_ascii_uppercase();
            buf.push(char);
            continue;
        }

        match char {
            ' ' | '-' | '_' => {
                buf.push(' ');
                if let Some((_, mut char)) = input_iter.next() {
                    char.make_ascii_uppercase();
                    buf.push(char)
                } else {
                    break;
                }
            }
            _ => {
                char.make_ascii_lowercase();
                buf.push(char)
            }
        }
    }

    buf
}

#[derive(Clone, Default)]
struct MenuThing {
    aria_current: String,
    href: String,
    size: String,
    variant: String,
}

#[derive(Clone, Default)]
struct MenuGroup {
    title: String,
    things: Vec<MenuThing>,
}

#[component]
pub fn TestButtons() -> impl IntoView {
    let menu_groups = vec![
        MenuGroup {
            title: "Inactive".to_string(),
            things: vec![
                MenuThing {
                    href: "/1".to_string(),
                    size: "small".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    href: "/2".to_string(),
                    size: "default".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    href: "/3".to_string(),
                    size: "large".to_string(),
                    ..Default::default()
                },
            ],
        },
        MenuGroup {
            title: "Active".to_string(),
            things: vec![
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/1".to_string(),
                    size: "small".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/2".to_string(),
                    size: "default".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/3".to_string(),
                    size: "large".to_string(),
                    ..Default::default()
                },
            ],
        },
        MenuGroup {
            title: "Outline Inactive".to_string(),
            things: vec![
                MenuThing {
                    href: "/1".to_string(),
                    size: "small".to_string(),
                    variant: "outline".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    href: "/2".to_string(),
                    size: "default".to_string(),
                    variant: "outline".to_string(),
                    ..Default::default()
                },
                MenuThing {
                    href: "/3".to_string(),
                    size: "large".to_string(),
                    variant: "outline".to_string(),
                    ..Default::default()
                },
            ],
        },
        MenuGroup {
            title: "Outline Active".to_string(),
            things: vec![
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/1".to_string(),
                    size: "small".to_string(),
                    variant: "outline".to_string(),
                },
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/2".to_string(),
                    size: "default".to_string(),
                    variant: "outline".to_string(),
                },
                MenuThing {
                    aria_current: "page".to_string(),
                    href: "/3".to_string(),
                    size: "large".to_string(),
                    variant: "outline".to_string(),
                },
            ],
        },
    ];

    let links = menu_groups.clone();
    let buttons = menu_groups;

    view! {
        <SidebarMenu>
            <For each=move || links.to_owned() key=|group| group.title.clone() let(group)>
                <SidebarGroup>
                    <SidebarGroupLabel>"Link "{group.title}</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <For
                            each=move || group.things.to_owned()
                            key=|thing| thing.size.to_string()
                            let(thing)
                        >
                            <SidebarMenuButton>
                                <Link
                                    aria_current=thing.aria_current.clone()
                                    href=thing.href
                                    size=thing.size.clone()
                                    variant=thing.variant.clone()
                                >
                                    {icon!(icondata::LuLink)}
                                    <span>{string_to_title(&thing.size)}</span>
                                </Link>
                            </SidebarMenuButton>
                        </For>
                    </SidebarGroupContent>
                </SidebarGroup>
            </For>
            <For each=move || buttons.to_owned() key=|group| group.title.clone() let(group)>
                <SidebarGroup>
                    <SidebarGroupLabel>"Button "{group.title}</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <For
                            each=move || group.things.to_owned()
                            key=|thing| thing.size.to_string()
                            let(thing)
                        >
                            <SidebarMenuButton>
                                <Button
                                    aria_current=thing.aria_current.clone()
                                    size=thing.size.clone()
                                    variant=thing.variant.clone()
                                >
                                    {icon!(icondata::MdiButtonPointer)}
                                    <span>{string_to_title(&thing.size)}</span>
                                </Button>
                            </SidebarMenuButton>
                        </For>
                    </SidebarGroupContent>
                </SidebarGroup>
            </For>
            <SidebarGroup>
                <SidebarGroupLabel>"Link Button"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <For
                        each=move || [
                            "default",
                            "secondary",
                            "outline",
                            "ghost",
                            "link",
                            "destructive",
                        ]
                        key=|variant| variant.to_string()
                        let(variant)
                    >
                        <SidebarMenuItem>
                            <SidebarMenuButton>
                                <Link href="/link-button" render_as="button" variant>
                                    {icon!(icondata::LuInfo)}
                                    <span>"Link Button"</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </For>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarMenu>
    }
}
