use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleOutlineExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-2">
            <Toggle variant="outline" aria_label="Toggle italic">
                {icon!(icondata::LuItalic)}
                <span>"Italic"</span>
            </Toggle>
            <Toggle variant="outline" aria_label="Toggle bold">
                {icon!(icondata::LuBold)}
                <span>"Bold"</span>
            </Toggle>
        </div>
    }
}
