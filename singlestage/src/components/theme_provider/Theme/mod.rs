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

#[derive(Clone)]
pub struct Theme {
    pub common: &'static str,
    pub dark: &'static str,
    pub light: &'static str,
}
