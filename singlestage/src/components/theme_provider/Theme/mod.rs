mod amber;
mod blue;
mod default;
mod lime;
mod mono;
mod orange;
mod purple;
mod red;
mod rose;
mod scaled;
mod teal;
mod violet;
mod yellow;

pub use amber::*;
pub use blue::*;
pub use default::*;
pub use lime::*;
pub use mono::*;
pub use orange::*;
pub use purple::*;
pub use red::*;
pub use rose::*;
pub use scaled::*;
pub use teal::*;
pub use violet::*;
pub use yellow::*;

use std::borrow::Cow;

#[cfg(not(feature = "islands"))]
#[derive(Clone)]
pub struct Theme {
    pub common: Cow<'static, str>,
    pub dark: Cow<'static, str>,
    pub light: Cow<'static, str>,
}

#[cfg(feature = "islands")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "islands")]
#[derive(Clone, Deserialize, Serialize)]
pub struct Theme {
    pub common: Cow<'static, str>,
    pub dark: Cow<'static, str>,
    pub light: Cow<'static, str>,
}
