use leptos::prelude::*;

mod button;
mod checkbox;
mod context_menu;
mod dropdown;
mod form_reset;
mod input;
mod label;
mod radio;
mod select;
mod slider;
mod textarea;
mod toggle;
mod tooltip;

pub use button::*;
pub use checkbox::*;
pub use context_menu::*;
pub use dropdown::*;
pub use form_reset::*;
pub use input::*;
pub use label::*;
pub use radio::*;
pub use select::*;
pub use slider::*;
pub use textarea::*;
pub use toggle::*;
pub use tooltip::*;

#[component]
pub fn DebugAll() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <DebugButton />
            <DebugCheckbox />
            <DebugContextMenu />
            <DebugDropdown />
            <DebugFormReset />
            <DebugInput />
            <DebugLabel />
            <DebugRadio />
            <DebugSelect />
            <DebugSlider />
            <DebugTextarea />
            <DebugToggle />
            <DebugTooltip />
        </div>
    }
}
