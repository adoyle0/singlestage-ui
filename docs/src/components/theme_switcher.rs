use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use singlestage::*;

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let theme_context = expect_context::<ThemeProviderContext>();
    let prefers_dark = RwSignal::new(false);
    let selected_theme = RwSignal::new("default".to_string());

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

            match theme_mode {
                "dark" => theme_context.mode.set(Mode::Dark),
                "light" => theme_context.mode.set(Mode::Light),
                _ => theme_context.mode.set(Mode::Auto),
            }

            if !theme.is_empty() {
                selected_theme.set(theme.to_string());
            }
        }
    });

    let swap_theme = move |_| {
        let mode;
        match theme_context.mode.get_untracked() {
            Mode::Auto => {
                if prefers_dark.get_untracked() {
                    theme_context.mode.set(Mode::Light);
                    mode = "light";
                } else {
                    theme_context.mode.set(Mode::Dark);
                    mode = "dark";
                }
            }
            Mode::Dark => {
                theme_context.mode.set(Mode::Light);
                mode = "light"
            }
            Mode::Light => {
                theme_context.mode.set(Mode::Dark);
                mode = "dark"
            }
        };

        let _ = document()
            .unchecked_ref::<web_sys::HtmlDocument>()
            .set_cookie(&format!("theme_mode={}; Path=/", mode).as_str());
    };

    Effect::new(move || {
        let _ = document()
            .unchecked_ref::<web_sys::HtmlDocument>()
            .set_cookie(&format!("theme={}; Path=/", selected_theme.get()).as_str());

        theme_context
            .theme
            .set(match selected_theme.get().as_str() {
                "amber" => Theme::Amber,
                "blue" => Theme::Blue,
                "default" => Theme::Default,
                "lime" => Theme::Lime,
                "mono" => Theme::Mono,
                "orange" => Theme::Orange,
                "purple" => Theme::Purple,
                "red" => Theme::Red,
                "rose" => Theme::Rose,
                "scaled" => Theme::Scaled,
                "teal" => Theme::Teal,
                "violet" => Theme::Violet,
                "yellow" => Theme::Yellow,
                _ => Theme::Default,
            })
    });

    view! {
        <span class="flex space-x-2">
            <Tooltip side="bottom" value="Select theme">
                <Select value=selected_theme class="h-8">
                    <optgroup label="Colors">
                        <option value="default">"Default"</option>
                        <option value="amber">"Amber"</option>
                        <option value="blue">"Blue"</option>
                        <option value="lime">"Lime"</option>
                        <option value="orange">"Orange"</option>
                        <option value="purple">"Purple"</option>
                        <option value="red">"Red"</option>
                        <option value="rose">"Rose"</option>
                        <option value="teal">"Teal"</option>
                        <option value="violet">"Violet"</option>
                        <option value="yellow">"Yellow"</option>
                    </optgroup>
                    <optgroup label="Layout">
                        <option value="mono">"Mono"</option>
                        <option value="scaled">"Scaled"</option>
                    </optgroup>
                </Select>
            </Tooltip>

            <Tooltip side="bottom" value="Toggle dark mode">
                <Button variant="outline" size="sm-icon" on:click=swap_theme>
                    <Show when=move || { theme_context.mode.get() == Mode::Light }>
                        <span>{icon!(icondata::LuSun)}</span>
                    </Show>
                    <Show when=move || { theme_context.mode.get() == Mode::Dark }>
                        <span>{icon!(icondata::LuMoon)}</span>
                    </Show>
                    <Show when=move || { theme_context.mode.get() == Mode::Auto }>
                        <span class="block dark:hidden">{icon!(icondata::LuSun)}</span>
                        <span class="hidden dark:block">{icon!(icondata::LuMoon)}</span>
                    </Show>
                </Button>
            </Tooltip>

            <Tooltip side="bottom" align="end" value="GitHub repository">
                <a href="https://github.com/adoyle0/singlestage-ui">
                    <Button size="sm-icon">{icon!(icondata::SiGithub)}</Button>
                </a>
            </Tooltip>
        </span>
    }
}
