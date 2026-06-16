use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
    nested_router::Outlet,
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
pub fn SidebarButton() -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant="ghost" size="sm-icon">
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

use leptos::wasm_bindgen::JsCast;
#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let theme_context = expect_context::<ThemeProviderContext>();
    let prefers_dark = RwSignal::new(false);
    let selected_theme = RwSignal::new("neutral".to_string());
    let selected_base = RwSignal::new("vega".to_string());

    // TODO: Make reactive
    Effect::new(move || {
        let media_query = window()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap_or(None);

        if let Some(media_query) = media_query.as_ref() {
            let matches = media_query.matches();

            prefers_dark.set(matches);
        }
    });

    Effect::new(move || {
        if let Ok(cookies) = document().unchecked_ref::<web_sys::HtmlDocument>().cookie() {
            let mut theme = "";
            let mut theme_mode = "";

            let _ = cookies
                .split(";")
                .map(|cookie| {
                    let split = cookie.split("=").collect::<Vec<&str>>();

                    match split[0].trim() {
                        "theme" => theme = split[1],
                        "theme_mode" => theme_mode = split[1],
                        _ => {}
                    }
                })
                .count();

            theme_context.mode.set(Mode::from(theme_mode));

            if !theme.is_empty() {
                selected_theme.set(theme.to_string());
            }
        }
    });

    let swap_theme = move |_| {
        match theme_context.mode.get_untracked() {
            Mode::Dark => {
                theme_context.mode.set(Mode::Light);
            }
            Mode::Light => {
                theme_context.mode.set(Mode::Dark);
            }
            _ => {
                if prefers_dark.get_untracked() {
                    theme_context.mode.set(Mode::Light);
                } else {
                    theme_context.mode.set(Mode::Dark);
                }
            }
        };

        let _ = document()
            .unchecked_ref::<web_sys::HtmlDocument>()
            .set_cookie(
                format!("theme_mode={}; Path=/", theme_context.mode.get_untracked()).as_str(),
            );
    };

    Effect::new(move || {
        let _ = document()
            .unchecked_ref::<web_sys::HtmlDocument>()
            .set_cookie(format!("theme={}; Path=/", selected_theme.get()).as_str());

        theme_context
            .theme
            .set(match selected_theme.get().as_str() {
                "amber" => Theme::Amber,
                "blue" => Theme::Blue,
                "lime" => Theme::Lime,
                "mono" => Theme::Mono,
                "neutral" => Theme::Neutral,
                "orange" => Theme::Orange,
                "purple" => Theme::Purple,
                "red" => Theme::Red,
                "rose" => Theme::Rose,
                "scaled" => Theme::Scaled,
                "teal" => Theme::Teal,
                "violet" => Theme::Violet,
                "yellow" => Theme::Yellow,
                _ => Theme::Neutral,
            })
    });

    Effect::new(move || {
        // let _ = document()
        //     .unchecked_ref::<web_sys::HtmlDocument>()
        //     .set_cookie(format!("theme={}; Path=/", selected_theme.get()).as_str());

        theme_context.base.set(match selected_base.get().as_str() {
            "luma" => ThemeBase::Luma,
            "lyra" => ThemeBase::Lyra,
            "maia" => ThemeBase::Maia,
            "mira" => ThemeBase::Mira,
            "nova" => ThemeBase::Nova,
            "sera" => ThemeBase::Sera,
            _ => ThemeBase::Vega,
        })
    });

    view! {
        <span class="flex space-x-2">
            <Tooltip>
                <TooltipTrigger>
                    <Select value=selected_base todo_name_me_size="sm">
                        <SelectOption value="luma">"Luma"</SelectOption>
                        <SelectOption value="lyra">"Lyra"</SelectOption>
                        <SelectOption value="maia">"Maia"</SelectOption>
                        <SelectOption value="mira">"Mira"</SelectOption>
                        <SelectOption value="nova">"Nova"</SelectOption>
                        <SelectOption value="sera">"Sera"</SelectOption>
                        <SelectOption value="vega">"Vega"</SelectOption>
                    </Select>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                    <p>"Select base theme"</p>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger>
                    <Select value=selected_theme todo_name_me_size="sm">
                        <SelectOptGroup label="Colors">
                            <SelectOption value="neutral">"Neutral"</SelectOption>
                            <SelectOption value="amber">"Amber"</SelectOption>
                            <SelectOption value="blue">"Blue"</SelectOption>
                            <SelectOption value="lime">"Lime"</SelectOption>
                            <SelectOption value="orange">"Orange"</SelectOption>
                            <SelectOption value="purple">"Purple"</SelectOption>
                            <SelectOption value="red">"Red"</SelectOption>
                            <SelectOption value="rose">"Rose"</SelectOption>
                            <SelectOption value="teal">"Teal"</SelectOption>
                            <SelectOption value="violet">"Violet"</SelectOption>
                            <SelectOption value="yellow">"Yellow"</SelectOption>
                        </SelectOptGroup>
                        <SelectOptGroup label="Layout">
                            <SelectOption value="mono">"Mono"</SelectOption>
                            <SelectOption value="scaled">"Scaled"</SelectOption>
                        </SelectOptGroup>
                    </Select>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                    <p>"Select color scheme"</p>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger>
                    <Button variant="outline" size="sm-icon" on:click=swap_theme>
                        {move || match theme_context.mode.get() {
                            Mode::Light => {
                                view! { <span>{icon!(icondata::LuSun)}</span> }.into_any()
                            }
                            Mode::Dark => {
                                view! { <span>{icon!(icondata::LuMoon)}</span> }.into_any()
                            }
                            _ => {
                                match prefers_dark.get() {
                                    false => {
                                        view! { <span>{icon!(icondata::LuSun)}</span> }.into_any()
                                    }
                                    true => {
                                        view! { <span>{icon!(icondata::LuMoon)}</span> }.into_any()
                                    }
                                }
                            }
                        }}
                    </Button>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                    <p>"Toggle dark mode"</p>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger>
                    <Link
                        href="https://github.com/adoyle0/singlestage-ui"
                        render_as="button"
                        size="sm-icon"
                    >
                        {icon!(icondata::SiGithub)}
                    </Link>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                    <p>"GitHub repository"</p>
                </TooltipContent>
            </Tooltip>
        </span>
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
                                <ParentRoute path=StaticSegment("") view=Outlet>
                                    <Route path=StaticSegment("/") view=Home />
                                    // TODO: Figure out why Trunk won't compile this
                                    // <Route path=StaticSegment("/all") view=DebugAll />
                                    <Route path=StaticSegment("/button") view=DebugButton />
                                    <Route path=StaticSegment("/checkbox") view=DebugCheckbox />
                                    <Route
                                        path=StaticSegment("/context_menu")
                                        view=DebugContextMenu
                                    />
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
                        </main>
                    </Router>
                </SidebarInset>
            </SidebarProvider>
        </ThemeProvider>
    }
}
