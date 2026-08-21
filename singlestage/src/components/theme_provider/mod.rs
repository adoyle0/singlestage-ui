mod mode;
pub use mode::*;

#[allow(non_snake_case)]
pub mod Theme;

#[allow(non_snake_case)]
pub mod ThemeBase;

use crate::{CSS, CSS_DARK};

use leptos::prelude::*;
use leptos_meta::Style;

#[derive(Clone)]
pub struct ThemeProviderContext {
    /// Set which theme base to use
    pub base: RwSignal<ThemeBase::ThemeBase>,
    /// Set the theme's light/dark mode behavior
    pub mode: RwSignal<Mode>,
    /// Set the current theme
    pub theme: RwSignal<Theme::Theme>,
}

/// Provides nonce support for inline styles. Returns `None` if the `nonce`
/// feature is not enabled or no nonce is in context.
fn request_nonce() -> Option<String> {
    #[cfg(feature = "nonce")]
    {
        leptos::nonce::use_nonce().map(|n| n.to_string())
    }
    #[cfg(not(feature = "nonce"))]
    {
        None
    }
}

/// Provides theme support to children. Note: Setting `mode` and `theme` here are only used for
/// initial values. Updates should be done via `ThemeProviderContext`.
#[component]
pub fn ThemeProviderInner(
    children: Children,
    /// Set the initial theme base Defaults to ThemeBase::Vega.
    #[prop(optional, into)]
    base: MaybeProp<ThemeBase::ThemeBase>,
    /// Set the initial light/dark mode behavior. Defaults to `auto`/`Mode::Auto`.
    ///
    /// Accepted values: `auto` | `dark` | `light` or a `Mode`
    #[prop(optional, into)]
    mode: MaybeProp<String>,
    /// Set the initial Theme.
    #[prop(optional, into)]
    theme: MaybeProp<Theme::Theme>,
) -> impl IntoView {
    let mode = RwSignal::<Mode>::new(mode.get_untracked().unwrap_or_default().into());
    let theme = RwSignal::new(theme.get_untracked().unwrap_or(Theme::Neutral));
    let base = RwSignal::new(base.get_untracked().unwrap_or(ThemeBase::Vega));

    let context = ThemeProviderContext { base, mode, theme };
    provide_context(context);

    // TODO: Consider slicing up the base theme and merging everything to reduce css
    // bloat/duplication
    view! {
        <Style id="singlestage">{CSS}</Style>
        <style
            id="theme-base"
            nonce=request_nonce()
            inner_html=move || {
                match mode.get() {
                    Mode::Dark => {
                        format!("{}{}", base.get().light.to_string(), base.get().dark.to_string())
                    }
                    Mode::Light => base.get().light.to_string(),
                    Mode::Auto => {
                        format!(
                            "{}\n\n@media (prefers-color-scheme: dark) {{{}}}",
                            base.get().light.to_string(),
                            base.get().dark.to_string(),
                        )
                    }
                }
            }
        ></style>
        <style
            id="theme"
            nonce=request_nonce()
            inner_html=move || {
                let theme = theme.get();
                match mode.get() {
                    Mode::Dark => {
                        format!(":root{{ {} {}}}\n{}\n", theme.common, theme.dark, CSS_DARK)
                    }
                    Mode::Light => format!(":root{{ {} {}}}\n", theme.common, theme.light),
                    Mode::Auto => {
                        format!(
                            ":root{{ {} {}}}\n\n@media (prefers-color-scheme: dark) {{{}\n  :root {{ {}  }}}}",
                            theme.common,
                            theme.light,
                            CSS_DARK,
                            theme.dark,
                        )
                    }
                }
            }
        ></style>
        {children()}
    }
}

#[cfg(not(feature = "islands"))]
#[component]
pub fn ThemeProvider(
    children: Children,
    /// Set the initial light/dark mode behavior. Defaults to `auto`/`Mode::Auto`.
    ///
    /// Accepted values: `auto` | `dark` | `light` or a `Mode`
    #[prop(optional, into)]
    mode: MaybeProp<String>,
    /// Set the initial Theme.
    #[prop(optional, into)]
    theme: MaybeProp<Theme::Theme>,
) -> impl IntoView {
    view! {
        <ThemeProviderInner mode theme>
            {children()}
        </ThemeProviderInner>
    }
}

#[cfg(feature = "islands")]
#[island]
pub fn ThemeProvider(
    children: Children,
    /// Set the initial light/dark mode behavior. Defaults to `auto`/`Mode::Auto`.
    ///
    /// Accepted values: `auto` | `dark` | `light` or a `Mode`
    #[prop(optional, into)]
    mode: Option<String>,
    /// Set the initial Theme.
    #[prop(optional, into)]
    theme: Option<Theme::Theme>,
) -> impl IntoView {
    use leptos_meta::provide_meta_context;

    provide_meta_context();
    view! {
        <ThemeProviderInner
            theme=theme.unwrap_or(Theme::Neutral)
            mode=mode.unwrap_or("auto".into())
        >
            {children()}
        </ThemeProviderInner>
    }
}
