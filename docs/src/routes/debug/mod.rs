use leptos::prelude::*;

mod checkbox;
mod form_reset;
mod input;
mod radio;
mod select;
mod slider;
mod textarea;

use checkbox::*;
use form_reset::*;
use input::*;
use radio::*;
use select::*;
use slider::*;
use textarea::*;

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
        </div>
    }
}
