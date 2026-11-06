use leptos::prelude::*;
use singlestage::*;

#[derive(Clone, Default)]
pub struct FileTreeItem {
    name: String,
    items: Vec<FileTreeItem>,
}

#[component]
pub fn RenderItem(file_item: FileTreeItem) -> impl IntoView {
    if !file_item.items.is_empty() {
        let open = RwSignal::new(false);

        view! {
            <Collapsible open>
                <CollapsibleTrigger>
                    <Button
                        variant="ghost"
                        size="sm"
                        class="w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground"
                    >
                        <span class=move || {
                            format!(
                                "p-0 transition-transform{}",
                                match open.get() {
                                    true => " rotate-90",
                                    false => "",
                                },
                            )
                        }>{icon!(icondata::LuChevronRight)}</span>
                        {icon!(icondata::LuFolder)}
                        {file_item.name}
                    </Button>
                </CollapsibleTrigger>
                <CollapsibleContent class="mt-1 ml-6">
                    <div class="flex flex-col gap-1">
                        <For
                            each=move || file_item.items.to_owned()
                            key=|file_item| file_item.name.clone()
                            let(file_item)
                        >
                            <RenderItem file_item />
                        </For>
                    </div>
                </CollapsibleContent>
            </Collapsible>
        }.into_any()
    } else {
        view! {
            <Button variant="link" size="sm" class="w-full justify-start gap-2 text-foreground">
                {icon!(icondata::LuFile)}
                <span>{file_item.name}</span>
            </Button>
        }
        .into_any()
    }
}

#[component]
pub fn CollapsibleFileTreeExample() -> impl IntoView {
    let file_tree: Vec<FileTreeItem> = vec![
        FileTreeItem {
            name: "components".to_string(),
            items: vec![
                FileTreeItem {
                    name: "ui".to_string(),
                    items: vec![
                        FileTreeItem {
                            name: "button.tsx".to_string(),
                            ..Default::default()
                        },
                        FileTreeItem {
                            name: "card.tsx".to_string(),
                            ..Default::default()
                        },
                        FileTreeItem {
                            name: "dialog.tsx".to_string(),
                            ..Default::default()
                        },
                        FileTreeItem {
                            name: "input.tsx".to_string(),
                            ..Default::default()
                        },
                        FileTreeItem {
                            name: "select.tsx".to_string(),
                            ..Default::default()
                        },
                        FileTreeItem {
                            name: "table.tsx".to_string(),
                            ..Default::default()
                        },
                    ],
                },
                FileTreeItem {
                    name: "login-form.tsx".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "register-form.tsx".to_string(),
                    ..Default::default()
                },
            ],
        },
        FileTreeItem {
            name: "lib".to_string(),
            items: vec![
                FileTreeItem {
                    name: "utils.ts".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "cn.ts".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "api.ts".to_string(),
                    ..Default::default()
                },
            ],
        },
        FileTreeItem {
            name: "hooks".to_string(),
            items: vec![
                FileTreeItem {
                    name: "use-media-query.ts".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "use-debounce.ts".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "use-local-storage.ts".to_string(),
                    ..Default::default()
                },
            ],
        },
        FileTreeItem {
            name: "types".to_string(),
            items: vec![
                FileTreeItem {
                    name: "index.d.ts".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "api.d.ts".to_string(),
                    ..Default::default()
                },
            ],
        },
        FileTreeItem {
            name: "public".to_string(),
            items: vec![
                FileTreeItem {
                    name: "favicon.ico".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "logo.svg".to_string(),
                    ..Default::default()
                },
                FileTreeItem {
                    name: "images".to_string(),
                    ..Default::default()
                },
            ],
        },
        FileTreeItem {
            name: "app.tsx".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: "layout.tsx".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: "globals.css".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: "package.json".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: "tsconfig.json".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: "README.md".to_string(),
            ..Default::default()
        },
        FileTreeItem {
            name: ".gitignore".to_string(),
            ..Default::default()
        },
    ];

    view! {
        <Card class="mx-auto w-full max-w-[16rem] gap-2" size="sm">
            <CardHeader>
                <Tabs value="explorer">
                    <TabsList class="w-full">
                        <TabsTrigger value="explorer">"Explorer"</TabsTrigger>
                        <TabsTrigger value="settings">"Outline"</TabsTrigger>
                    </TabsList>
                </Tabs>
            </CardHeader>
            <CardContent>
                <div class="flex flex-col gap-1">
                    <For
                        each=move || file_tree.to_owned()
                        key=|file_item| file_item.name.clone()
                        let(file_item)
                    >
                        <RenderItem file_item />
                    </For>
                </div>
            </CardContent>
        </Card>
    }
}
