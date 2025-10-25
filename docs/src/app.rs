use crate::components::*;
use crate::routes::*;
use docs_macro::*;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Outlet, ParentRoute, Route, Router, Routes},
};
use singlestage::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="apple-touch-icon" sizes="57x57" href="/apple-icon-57x57.png" />
                <link rel="apple-touch-icon" sizes="60x60" href="/apple-icon-60x60.png" />
                <link rel="apple-touch-icon" sizes="72x72" href="/apple-icon-72x72.png" />
                <link rel="apple-touch-icon" sizes="76x76" href="/apple-icon-76x76.png" />
                <link rel="apple-touch-icon" sizes="114x114" href="/apple-icon-114x114.png" />
                <link rel="apple-touch-icon" sizes="120x120" href="/apple-icon-120x120.png" />
                <link rel="apple-touch-icon" sizes="144x144" href="/apple-icon-144x144.png" />
                <link rel="apple-touch-icon" sizes="152x152" href="/apple-icon-152x152.png" />
                <link rel="apple-touch-icon" sizes="180x180" href="/apple-icon-180x180.png" />
                <link
                    rel="icon"
                    type="image/png"
                    sizes="192x192"
                    href="/android-icon-192x192.png"
                />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="96x96" href="/favicon-96x96.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
                <link rel="manifest" href="/manifest.json" />
                <meta name="msapplication-TileColor" content="#ffffff" />
                <meta name="msapplication-TileImage" content="/ms-icon-144x144.png" />
                <meta name="theme-color" content="#ffffff" />
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

// Generates ComponentRoutes component
generate_component_routes!();

#[component]
pub fn NotFound() -> impl IntoView {
    view! { "Page not found." }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Singlestage UI" />
        <Stylesheet id="leptos" href="/pkg/singlestage_docs.css" />
        <ThemeProvider>
            <Router>
                <Routes fallback=NotFound>
                    <ParentRoute path=StaticSegment("") view=SidebarContainer>
                        <Route path=StaticSegment("/") view=LandingPage />
                        <Route path=StaticSegment("/introduction") view=Introduction />
                        <Route path=StaticSegment("/install") view=Installation />
                        <Route path=StaticSegment("/theme-provider") view=ThemeProviderRoute />
                        <Route path=StaticSegment("/icon-macro") view=IconMacroRoute />
                        <Route
                            path=StaticSegment("/reactive-debug")
                            view=|| {
                                view! {
                                    {if cfg!(debug_assertions) {
                                        ReactiveDebug.into_any()
                                    } else {
                                        NotFound.into_any()
                                    }}
                                }
                            }
                        />
                        <ComponentRoutes />
                    </ParentRoute>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}
