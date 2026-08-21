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
use std::borrow::Cow;

#[cfg(not(feature = "islands"))]
#[derive(Clone)]
pub struct ThemeBase {
    pub dark: Cow<'static, str>,
    pub light: Cow<'static, str>,
}

#[cfg(feature = "islands")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "islands")]
#[derive(Clone, Deserialize, Serialize)]
pub struct ThemeBase {
    pub dark: Cow<'static, str>,
    pub light: Cow<'static, str>,
}

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_luma")]
pub const Luma: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_LUMA_DARK),
    light: Cow::Borrowed(CSS_LUMA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_lyra")]
pub const Lyra: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_LYRA_DARK),
    light: Cow::Borrowed(CSS_LYRA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_maia")]
pub const Maia: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_MAIA_DARK),
    light: Cow::Borrowed(CSS_MAIA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_mira")]
pub const Mira: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_MIRA_DARK),
    light: Cow::Borrowed(CSS_MIRA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_nova")]
pub const Nova: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_NOVA_DARK),
    light: Cow::Borrowed(CSS_NOVA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_sera")]
pub const Sera: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_SERA_DARK),
    light: Cow::Borrowed(CSS_SERA),
};

#[allow(non_upper_case_globals)]
#[cfg(feature = "style_vega")]
pub const Vega: ThemeBase = ThemeBase {
    dark: Cow::Borrowed(CSS_VEGA_DARK),
    light: Cow::Borrowed(CSS_VEGA),
};
