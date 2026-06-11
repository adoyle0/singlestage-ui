use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
    nested_router::Outlet,
};

mod debug;
mod routes;

use debug::*;
use routes::*;
use singlestage::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn SidebarButton() -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button
                    variant="ghost"
                    size="sm-icon"
                    class="flex gap-2 justify-between items-center px-4 w-full h-14"
                >
                    <Show
                        when=move || { !sidebar.open.get() }
                        fallback=move || match sidebar.side.get().as_str() {
                            "right" => view! { {icon!(icondata::LuPanelRightClose)} }.into_any(),
                            _ => view! { {icon!(icondata::LuPanelLeftClose)} }.into_any(),
                        }
                    >
                        {match sidebar.side.get().as_str() {
                            "right" => view! { {icon!(icondata::LuPanelRightOpen)} }.into_any(),
                            _ => view! { {icon!(icondata::LuPanelLeftOpen)} }.into_any(),
                        }}
                    </Show>
                </Button>
            </TooltipTrigger>
            <TooltipContent side="bottom" align="start">
                <p>"Toggle sidebar"</p>
            </TooltipContent>
        </Tooltip>
    }
}

#[component]
pub fn SidebarComponent() -> impl IntoView {
    let routes = StoredValue::new([
        // "All",
        "Button",
        "Checkbox",
        "Context Menu",
        "Dropdown",
        "Form Reset",
        "Input",
        "Label",
        "Radio",
        "Select",
        "Slider",
        "Textarea",
        "Toggle",
        "Tooltip",
    ]);

    view! {
        <SidebarProvider>
            <Sidebar>
                <SidebarHeader>
                    <a href="/">
                        <h1 class="text-2xl font-semibold">
                            "test_site"
                            {if cfg!(feature = "csr") {
                                " csr"
                            } else if cfg!(feature = "ssr") {
                                " ssr"
                            } else {
                                ""
                            }}
                        </h1>
                    </a>
                </SidebarHeader>
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupLabel>"Components"</SidebarGroupLabel>
                        <SidebarGroupContent>
                            <SidebarMenu>
                                <For
                                    each=move || routes.get_value()
                                    key=|route| route.to_owned()
                                    let(route)
                                >
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Link href=format!(
                                                "/{}",
                                                &route.to_lowercase().replace(" ", "_"),
                                            )>{route}</Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </For>
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
            </Sidebar>
            <SidebarInset>
                <header class="flex sticky inset-x-0 top-0 z-10 items-center border-b bg-(--background) isolate shrink-0">
                    <SidebarTrigger>
                        <SidebarButton />
                    </SidebarTrigger>
                </header>

                <div class="my-8 mx-2 sm:mx-12 max-w-4xl">
                    <Outlet />
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // This tells the client where to find the server in csr builds
    #[cfg(feature = "csr")]
    leptos::server_fn::client::set_server_url("http://localhost:3000");

    view! {
        <Title text="test_site" />

        <ThemeProvider>
            {
                #[cfg(not(feature = "csr"))]
                // In csr mode the css file is sourced in index.html
                view! { <leptos_meta::Stylesheet id="leptos" href="/pkg/test_site.css" /> }
            } <Router>
                <Routes fallback=NotFound>
                    <ParentRoute path=StaticSegment("") view=SidebarComponent>
                        <Route path=StaticSegment("/") view=Home />
                        // TODO: Figure out why Trunk won't compile this
                        // <Route path=StaticSegment("/all") view=DebugAll />
                        <Route path=StaticSegment("/button") view=DebugButton />
                        <Route path=StaticSegment("/checkbox") view=DebugCheckbox />
                        <Route path=StaticSegment("/context_menu") view=DebugContextMenu />
                        <Route path=StaticSegment("/dropdown") view=DebugDropdown />
                        <Route path=StaticSegment("/form_reset") view=DebugFormReset />
                        <Route path=StaticSegment("/input") view=DebugInput />
                        <Route path=StaticSegment("/label") view=DebugLabel />
                        <Route path=StaticSegment("/radio") view=DebugRadio />
                        <Route path=StaticSegment("/select") view=DebugSelect />
                        <Route path=StaticSegment("/slider") view=DebugSlider />
                        <Route path=StaticSegment("/textarea") view=DebugTextarea />
                        <Route path=StaticSegment("/toggle") view=DebugToggle />
                        <Route path=StaticSegment("/tooltip") view=DebugTooltip />
                    </ParentRoute>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}
