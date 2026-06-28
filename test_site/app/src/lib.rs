use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

mod components;
mod routes;

use components::*;
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
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // This tells the client where to find the server in csr builds
    #[cfg(feature = "csr")]
    leptos::server_fn::client::set_server_url("http://localhost:3000");

    view! {
        <Title text="test_site" />

        <ThemeProvider>
            <SidebarProvider>
                <AppSidebar />
                <SidebarInset>
                    <Router>
                        <header class="flex sticky inset-x-0 top-0 z-10 items-center border-b bg-(--background) isolate shrink-0">
                            <div class="flex gap-2 justify-between items-center px-4 w-full h-14">
                                <SidebarTrigger>
                                    <SidebarButton />
                                </SidebarTrigger>
                                <ThemeSwitcher />
                            </div>
                        </header>

                        <main class="my-8 mx-2 sm:mx-12 max-w-4xl">
                            {
                                #[cfg(not(feature = "csr"))]
                                // In csr mode the css file is sourced in index.html
                                view! {
                                    <leptos_meta::Stylesheet
                                        id="leptos"
                                        href="/pkg/test_site.css"
                                    />
                                }
                            } <Routes fallback=NotFound>
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
                            </Routes>
                        </main>
                    </Router>
                </SidebarInset>
            </SidebarProvider>
        </ThemeProvider>
    }
}
