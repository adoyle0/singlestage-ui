use leptos::prelude::*;

mod button;
mod checkbox;
mod context_menu;
mod dropdown;
mod form_reset;
mod input;
mod radio;
mod select;
mod slider;
mod textarea;
mod toggle;

use button::*;
use checkbox::*;
use context_menu::*;
use dropdown::*;
use form_reset::*;
use input::*;
use radio::*;
use select::*;
use slider::*;
use textarea::*;
use toggle::*;

#[component]
pub fn ReactiveDebug() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <DebugCheckbox />
            <DebugInput />
            <DebugTextarea />
            <DebugFormReset />
            <DebugSelect />
            <DebugRadio />
            <DebugSlider />
            <DebugButton />
            <DebugToggle />
            <DebugDropdown />
            <DebugContextMenu />
        </div>
    }
}
