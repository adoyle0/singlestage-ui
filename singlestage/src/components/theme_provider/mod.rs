mod mode;
pub use mode::*;

#[allow(non_snake_case)]
pub mod Theme;

use crate::{CSS, CSS_DARK};
#[cfg(feature = "style_luma")]
use crate::{CSS_LUMA, CSS_LUMA_DARK};
#[cfg(feature = "style_lyra")]
use crate::{CSS_LYRA, CSS_LYRA_DARK};
#[cfg(feature = "style_maia")]
use crate::{CSS_MAIA, CSS_MAIA_DARK};
#[cfg(feature = "style_mira")]
use crate::{CSS_MIRA, CSS_MIRA_DARK};
#[cfg(feature = "style_nova")]
use crate::{CSS_NOVA, CSS_NOVA_DARK};
#[cfg(feature = "style_sera")]
use crate::{CSS_SERA, CSS_SERA_DARK};
#[cfg(feature = "style_vega")]
use crate::{CSS_VEGA, CSS_VEGA_DARK};

use leptos::prelude::*;
use leptos_meta::Style;

#[derive(Clone)]
pub struct ThemeProviderContext {
    /// Set the theme's light/dark mode behavior. Defaults to `Mode::Auto`.
    pub mode: RwSignal<Mode>,
    /// The get/set/update the current theme in use.
    pub theme: RwSignal<Theme::Theme>,
}

/// Provides theme support to children. Note: Setting `mode` and `theme` here are only used for
/// initial values. Updates should be done via `ThemeProviderContext`.
#[component]
pub fn ThemeProviderInner(
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
    let mode = RwSignal::<Mode>::new(mode.get_untracked().unwrap_or_default().into());
    let theme = RwSignal::new(theme.get_untracked().unwrap_or(Theme::Neutral));

    let context = ThemeProviderContext { theme, mode };
    provide_context(context);

    // TODO: Consider slicing up the base theme and merging everything to reduce css
    // bloat/duplication
    view! {
        <Style id="singlestage">{CSS}</Style>
        <style
            id="theme-base"
            inner_html=move || {
                match mode.get() {
                    Mode::Dark => format!("{}{}", CSS_VEGA, CSS_VEGA_DARK),
                    Mode::Light => CSS_VEGA.to_string(),
                    Mode::Auto => {
                        format!(
                            "{}\n\n@media (prefers-color-scheme: dark) {{{}}}",
                            CSS_VEGA,
                            CSS_VEGA_DARK,
                        )
                    }
                }
            }
        ></style>
        <style
            id="theme"
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
