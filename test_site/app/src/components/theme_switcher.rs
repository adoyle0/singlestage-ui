use leptos::{prelude::*, wasm_bindgen::JsCast};
use singlestage::*;

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
